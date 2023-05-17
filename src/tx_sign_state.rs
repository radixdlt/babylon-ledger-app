use nanos_sdk::io::Comm;
use sbor::bech32::network::NetworkId;
use sbor::decoder_error::DecoderError;
use sbor::instruction_extractor::InstructionExtractor;
use sbor::print::instruction_printer::InstructionPrinter;
use sbor::print::tty::TTY;
use sbor::sbor_decoder::*;

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::{KeyPair25519, ED25519_PUBLIC_KEY_LEN, ED25519_SIGNATURE_LEN};
use crate::crypto::hash::{Blake2bHasher, Digest};
use crate::crypto::secp256k1::{
    KeyPairSecp256k1, SECP256K1_PUBLIC_KEY_LEN, SECP256K1_SIGNATURE_LEN,
};
use crate::ledger_display_io::LedgerTTY;
use crate::ui::multiline_scroller::MultilineMessageScroller;
use crate::ui::multipage_validator::MultipageValidator;
use crate::ui::single_message::SingleMessage;
use crate::utilities::conversion::{lower_as_hex, upper_as_hex};
use crate::utilities::max::max;

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum SignTxType {
    Ed25519,
    Ed25519Summary,
    Secp256k1,
    Secp256k1Summary,
    AuthEd25519,
    AuthSecp256k1,
}

#[repr(align(4))]
struct SignFlowState {
    sign_type: SignTxType,
    tx_packet_count: u32,
    tx_size: usize,
    path: Bip32Path,
    hasher: Blake2bHasher,
}

impl SignFlowState {
    fn process_data(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignTxType,
    ) -> Result<(), AppError> {
        match class {
            CommandClass::Regular => {
                let path = Bip32Path::read(comm).and_then(|path| match tx_type {
                    SignTxType::Ed25519 | SignTxType::Ed25519Summary | SignTxType::AuthEd25519 => {
                        path.validate()
                    }
                    SignTxType::Secp256k1
                    | SignTxType::Secp256k1Summary
                    | SignTxType::AuthSecp256k1 => path.validate_olympia_path(),
                })?;
                self.start(tx_type, path)?;
                self.update_counters(0); // First packet contains no data
                Ok(())
            }

            CommandClass::Continuation | CommandClass::LastData => {
                self.validate(class, tx_type)?;
                let data = comm.get_data()?;
                self.update_counters(data.len());

                match tx_type {
                    SignTxType::Ed25519
                    | SignTxType::Ed25519Summary
                    | SignTxType::Secp256k1
                    | SignTxType::Secp256k1Summary => self.hasher.update(data),
                    SignTxType::AuthEd25519 | SignTxType::AuthSecp256k1 => Ok(()),
                }
            }

            CommandClass::Unknown => Err(AppError::BadTxSignSequence),
        }
    }

    pub fn network_id(&mut self) -> Result<NetworkId, AppError> {
        self.path.network_id()
    }

    fn finalize(&mut self) -> Result<Digest, AppError> {
        self.hasher.finalize()
    }

    fn reset(&mut self) {
        self.hasher.reset();
        self.tx_packet_count = 0;
        self.tx_size = 0;
        self.sign_type = SignTxType::Ed25519;
        self.path = Bip32Path::new(0);
    }

    fn partial_reset(&mut self) {
        self.hasher.reset();
        self.tx_packet_count = 0;
        self.tx_size = 0;
    }

    fn start(&mut self, sign_type: SignTxType, path: Bip32Path) -> Result<(), AppError> {
        self.partial_reset();
        self.sign_type = sign_type;
        self.path = path;
        self.hasher.init()
    }

    fn sign_started(&self) -> bool {
        self.tx_packet_count != 0
    }

    fn validate(&self, class: CommandClass, sign_type: SignTxType) -> Result<(), AppError> {
        if self.sign_started() {
            self.validate_intermediate(class, sign_type)
        } else {
            self.validate_initial(class)
        }
    }

    fn validate_intermediate(
        &self,
        class: CommandClass,
        sign_type: SignTxType,
    ) -> Result<(), AppError> {
        if self.sign_type != sign_type {
            return Err(AppError::BadTxSignType);
        }

        match class {
            CommandClass::Continuation | CommandClass::LastData => Ok(()),
            _ => Err(AppError::BadTxSignSequence),
        }
    }

    fn validate_initial(&self, class: CommandClass) -> Result<(), AppError> {
        if class != CommandClass::Regular {
            return Err(AppError::BadTxSignSequence);
        }

        Ok(())
    }

    fn sign_tx(&self, tx_type: SignTxType, digest: Digest) -> Result<SignOutcome, AppError> {
        match tx_type {
            SignTxType::Ed25519 | SignTxType::Ed25519Summary | SignTxType::AuthEd25519 => {
                KeyPair25519::derive(&self.path).and_then(|keypair| {
                    keypair
                        .sign(digest.as_bytes())
                        .map(|signature| SignOutcome::SignatureEd25519 {
                            signature,
                            key: keypair.public_key(),
                            digest: digest.0,
                        })
                })
            }

            SignTxType::Secp256k1 | SignTxType::Secp256k1Summary | SignTxType::AuthSecp256k1 => {
                KeyPairSecp256k1::derive(&self.path).and_then(|keypair| {
                    keypair.sign(digest.as_bytes()).map(|signature| {
                        SignOutcome::SignatureSecp256k1 {
                            signature,
                            key: keypair.public_key(),
                            digest: digest.0,
                        }
                    })
                })
            }
        }
    }

    fn update_counters(&mut self, size: usize) {
        self.tx_size += size;
        self.tx_packet_count += 1;
    }
}

const MAX_SIGNATURE_SIZE: usize = max(ED25519_SIGNATURE_LEN, SECP256K1_SIGNATURE_LEN);
const MAX_PUBKEY_SIZE: usize = max(ED25519_PUBLIC_KEY_LEN, SECP256K1_PUBLIC_KEY_LEN);
const MAX_SIGNATURE_PAYLOAD: usize = MAX_SIGNATURE_SIZE + MAX_PUBKEY_SIZE;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum SignOutcome {
    SendNextPacket,
    SigningRejected,
    SignatureEd25519 {
        signature: [u8; ED25519_SIGNATURE_LEN],
        key: [u8; ED25519_PUBLIC_KEY_LEN],
        digest: [u8; Digest::DIGEST_LENGTH],
    },
    SignatureSecp256k1 {
        signature: [u8; SECP256K1_SIGNATURE_LEN],
        key: [u8; SECP256K1_PUBLIC_KEY_LEN],
        digest: [u8; Digest::DIGEST_LENGTH],
    },
}

pub struct InstructionProcessor {
    state: SignFlowState,
    extractor: InstructionExtractor,
    printer: InstructionPrinter,
}

impl InstructionProcessor {
    pub fn sign_tx(&self, tx_type: SignTxType, digest: Digest) -> Result<SignOutcome, AppError> {
        self.state.sign_tx(tx_type, digest)
    }

    pub fn tx_size(&self) -> usize {
        self.state.tx_size
    }

    pub fn set_network(&mut self) -> Result<(), AppError> {
        match self.state.sign_type {
            SignTxType::Ed25519 | SignTxType::Ed25519Summary | SignTxType::AuthEd25519 => {
                self.printer.set_network(self.state.network_id()?)
            }
            SignTxType::Secp256k1 | SignTxType::Secp256k1Summary | SignTxType::AuthSecp256k1 => {
                self.printer.set_network(NetworkId::OlympiaMainNet)
            }
        };
        Ok(())
    }

    pub fn set_show_instructions(&mut self) {
        match self.state.sign_type {
            SignTxType::Secp256k1Summary
            | SignTxType::Ed25519Summary
            | SignTxType::AuthEd25519
            | SignTxType::AuthSecp256k1 => {
                self.printer.set_show_instructions(false);
            }
            SignTxType::Secp256k1 | SignTxType::Ed25519 => {
                self.printer.set_show_instructions(true);
            }
        };
    }

    pub fn set_tty(&mut self, tty: TTY) {
        self.printer.set_tty(tty);
    }

    pub fn reset(&mut self) {
        self.state.reset();
        self.extractor.reset();
        self.printer.reset();
    }

    pub fn finalize(&mut self) -> Result<Digest, AppError> {
        self.state.finalize()
    }
}

pub struct TxSignState {
    decoder: SborDecoder,
    processor: InstructionProcessor,
    show_digest: bool,
}

impl SborEventHandler for InstructionProcessor {
    fn handle(&mut self, evt: SborEvent) {
        self.extractor.handle_event(&mut self.printer, evt);
    }
}

impl TxSignState {
    pub fn new() -> Self {
        Self {
            decoder: SborDecoder::new(true),
            processor: InstructionProcessor {
                state: SignFlowState {
                    sign_type: SignTxType::Ed25519,
                    tx_packet_count: 0,
                    tx_size: 0,
                    path: Bip32Path::new(0),
                    hasher: Blake2bHasher::new(),
                },
                extractor: InstructionExtractor::new(),
                printer: InstructionPrinter::new(NetworkId::LocalNet, LedgerTTY::new_tty()),
            },
            show_digest: false,
        }
    }

    pub fn reset(&mut self) {
        self.processor.reset();
        self.decoder.reset();
        self.processor.set_tty(LedgerTTY::new_tty());
    }

    pub fn process_request(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignTxType,
    ) -> Result<SignOutcome, AppError> {
        let result = self.do_process(comm, class, tx_type);

        match result {
            Ok(outcome) => match outcome {
                SignOutcome::SendNextPacket => result,
                _ => {
                    self.reset(); // Ensure state is reset on error or end of processing
                    result
                }
            },
            Err(_) => {
                self.reset(); // Ensure state is reset on error
                result
            }
        }
    }

    fn do_process(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignTxType,
    ) -> Result<SignOutcome, AppError> {
        self.processor.state.process_data(comm, class, tx_type)?;

        if class == CommandClass::Regular {
            self.processor.set_network()?;
            self.processor.set_show_instructions();
            self.show_digest = match tx_type {
                SignTxType::Ed25519 | SignTxType::Secp256k1 => comm.get_apdu_metadata().p1 == 1,
                SignTxType::Ed25519Summary | SignTxType::Secp256k1Summary => true,
                SignTxType::AuthEd25519 | SignTxType::AuthSecp256k1 => false,
            };
            self.show_introductory_screen(tx_type)?;
        } else {
            match tx_type {
                SignTxType::AuthEd25519 | SignTxType::AuthSecp256k1 => {
                    return if class != CommandClass::LastData {
                        Err(AppError::BadAuthSignSequence)
                    } else {
                        self.process_sign_auth(comm.get_data()?, tx_type)
                    }
                }
                _ => self.decode_tx_intent(comm.get_data()?, class)?,
            }
        }

        if class == CommandClass::LastData {
            self.finalize_sign_tx(tx_type)
        } else {
            Ok(SignOutcome::SendNextPacket)
        }
    }

    fn show_introductory_screen(&mut self, tx_type: SignTxType) -> Result<(), AppError> {
        let text = match tx_type {
            SignTxType::Ed25519
            | SignTxType::Secp256k1
            | SignTxType::Ed25519Summary
            | SignTxType::Secp256k1Summary => "Review\nSign\nTransaction",
            SignTxType::AuthEd25519 | SignTxType::AuthSecp256k1 => "Review\nSign\nAuthentication",
        };

        SingleMessage::new(text, false).show_and_wait();

        Ok(())
    }

    const NONCE_LENGTH: usize = 32;
    const DAPP_ADDRESS_LENGTH: usize = 70;
    const ORIGIN_LENGTH: usize = 150;
    const MIN_DAPP_ADDRESS_LENGTH: usize = 2; // 1 byte length + 1 byte address
    const MIN_ORIGIN_LENGTH: usize = 10; // 1 byte length + "https://a"
    const MIN_VALID_LENGTH: usize =
        Self::NONCE_LENGTH + Self::MIN_DAPP_ADDRESS_LENGTH + Self::MIN_ORIGIN_LENGTH;

    fn process_sign_auth(
        &mut self,
        value: &[u8],
        tx_type: SignTxType,
    ) -> Result<SignOutcome, AppError> {
        if value.len() < Self::MIN_VALID_LENGTH {
            return Err(AppError::BadAuthSignRequest);
        }

        let nonce = &value[..Self::NONCE_LENGTH];
        let addr_start = Self::NONCE_LENGTH + 1;
        let addr_end = addr_start + value[Self::NONCE_LENGTH] as usize;
        let address = &value[addr_start..addr_end];
        let hash_address = &value[Self::NONCE_LENGTH..addr_end];
        let origin = &value[addr_end..];

        let mut nonce_hex = [0u8; Self::NONCE_LENGTH * 2];

        for (i, &byte) in nonce.iter().enumerate() {
            nonce_hex[i * 2] = upper_as_hex(byte);
            nonce_hex[i * 2 + 1] = lower_as_hex(byte);
        }

        self.info_message(b"Origin:", origin);
        self.info_message(b"dApp Address:", address);
        self.info_message(b"Nonce:", &nonce_hex);

        let rc = MultipageValidator::new(&[&"Sign Auth?"], &[&"Accept"], &[&"Reject"]).ask();

        if rc {
            let digest = self.calculate_auth(nonce, hash_address, origin)?;
            self.processor.sign_tx(tx_type, digest)
        } else {
            return Ok(SignOutcome::SigningRejected);
        }
    }

    fn info_message(&mut self, title: &[u8], message: &[u8]) {
        MultilineMessageScroller::with_title(
            core::str::from_utf8(title).unwrap(),
            core::str::from_utf8(message).unwrap(),
        )
        .event_loop();
    }

    fn calculate_auth(
        &mut self,
        nonce: &[u8],
        address: &[u8],
        origin: &[u8],
    ) -> Result<Digest, AppError> {
        self.processor
            .state
            .hasher
            .calculate_auth(nonce, address, origin)
    }

    fn finalize_sign_tx(&mut self, tx_type: SignTxType) -> Result<SignOutcome, AppError> {
        let digest = self.processor.state.finalize()?;

        if self.show_digest {
            self.info_message(b"Digest:", &digest.as_hex());
        }

        let rc = MultipageValidator::new(&[&"Sign Tx?"], &[&"Accept"], &[&"Reject"]).ask();

        if rc {
            self.processor.sign_tx(tx_type, digest)
        } else {
            return Ok(SignOutcome::SigningRejected);
        }
    }

    fn decode_tx_intent(&mut self, data: &[u8], class: CommandClass) -> Result<(), AppError> {
        let result = self.call_decoder(data);

        match result {
            Ok(outcome) => self.validate_outcome(class, outcome),
            Err(err) => Err(err.into()),
        }
    }

    fn validate_outcome(
        &mut self,
        class: CommandClass,
        outcome: DecodingOutcome,
    ) -> Result<(), AppError> {
        match (outcome, class) {
            // Decoding done and it was last data packet
            (DecodingOutcome::Done(size), CommandClass::LastData)
                if size == self.processor.tx_size() =>
            {
                Ok(())
            }
            // Decoding is incomplete and it was first packet or continuation packet
            (DecodingOutcome::NeedMoreData(size), CommandClass::Regular)
            | (DecodingOutcome::NeedMoreData(size), CommandClass::Continuation)
                if size == self.processor.tx_size() =>
            {
                Ok(())
            }
            // All other combinations are invalid
            _ => Err(AppError::BadTxSignLen),
        }
    }

    fn call_decoder(&mut self, data: &[u8]) -> Result<DecodingOutcome, DecoderError> {
        self.decoder.decode(&mut self.processor, data)
    }
}

// Helper method to debug printing directly on device
#[cfg(test)]
pub fn print_debug(text: &str) {
    ui::MessageScroller::new(text).event_loop();
}
