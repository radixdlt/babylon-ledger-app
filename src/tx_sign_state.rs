use core::cmp::max;

use nanos_sdk::io::Comm;
use nanos_ui::ui;
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
use crate::crypto::hash::{Digest, HashType, Hasher};
use crate::crypto::secp256k1::{
    KeyPairSecp256k1, SECP256K1_PUBLIC_KEY_LEN, SECP256K1_SIGNATURE_LEN,
};
use crate::ledger_display_io::LedgerTTY;

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum SignTxType {
    None,
    Ed25519,
    Secp256k1,
}

struct SignFlowState {
    sign_type: SignTxType,
    tx_packet_count: u32,
    tx_size: usize,
    path: Bip32Path,
    hasher: Hasher,
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
                let path = Bip32Path::read(comm).and_then(|path| {
                    if tx_type == SignTxType::Ed25519 {
                        path.validate()
                    } else {
                        path.validate_olympia_path()
                    }
                })?;
                self.start(tx_type, path)?;
                self.update_counters(0); // First packet contains no data
                Ok(())
            }

            CommandClass::Continuation | CommandClass::LastData => {
                self.validate(class, tx_type)?;
                let data = comm.get_data()?;
                self.update_counters(data.len());
                self.hasher.update(data)
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
        self.sign_type = SignTxType::None;
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

        let hash_type = match sign_type {
            SignTxType::Ed25519 => HashType::Blake2b,
            SignTxType::Secp256k1 => HashType::SHA256,
            SignTxType::None => {
                return Err(AppError::BadTxSignRequestedState);
            }
        };

        self.hasher.init(hash_type)
    }

    fn sign_started(&self) -> bool {
        self.sign_type != SignTxType::None && self.tx_packet_count != 0
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
            _ => return Err(AppError::BadTxSignSequence),
        }
    }

    fn validate_initial(&self, class: CommandClass) -> Result<(), AppError> {
        if self.sign_type != SignTxType::None {
            return Err(AppError::BadTxSignInitialState);
        }

        if class != CommandClass::Regular {
            return Err(AppError::BadTxSignSequence);
        }

        Ok(())
    }

    fn sign_tx(&self, tx_type: SignTxType, digest: Digest) -> Result<SignOutcome, AppError> {
        self.validate_digest(tx_type, digest)?;

        let second_pass_digest = match tx_type {
            SignTxType::Ed25519 => Hasher::one_step(digest.as_bytes(), HashType::SHA512),
            SignTxType::Secp256k1 => Hasher::one_step(digest.as_bytes(), HashType::SHA256),
            _ => return Err(AppError::BadTxSignType),
        }?;

        match tx_type {
            SignTxType::None => return Err(AppError::BadTxSignStart),
            SignTxType::Ed25519 => KeyPair25519::derive(&self.path).and_then(|keypair| {
                keypair
                    .sign(second_pass_digest.as_bytes())
                    .map(|signature| SignOutcome::SignatureEd25519 {
                        signature,
                        key: keypair.public_key(),
                    })
            }),

            SignTxType::Secp256k1 => KeyPairSecp256k1::derive(&self.path).and_then(|keypair| {
                keypair
                    .sign(second_pass_digest.as_bytes())
                    .map(|signature| SignOutcome::SignatureSecp256k1 {
                        signature,
                        key: keypair.public_key(),
                    })
            }),
        }
    }

    fn update_counters(&mut self, size: usize) {
        self.tx_size += size;
        self.tx_packet_count += 1;
    }

    fn validate_digest(&self, tx_type: SignTxType, digest: Digest) -> Result<(), AppError> {
        match (tx_type, digest.hash_type()) {
            (SignTxType::Ed25519, HashType::Blake2b) => Ok(()),
            (SignTxType::Secp256k1, HashType::SHA256) => Ok(()),
            _ => Err(AppError::BadTxSignDigestState),
        }
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
    },
    SignatureSecp256k1 {
        signature: [u8; SECP256K1_SIGNATURE_LEN],
        key: [u8; SECP256K1_PUBLIC_KEY_LEN],
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

    pub fn display_hex_string(&mut self, data: &[u8]) {
        self.printer.display_hex_string(data);
    }

    pub fn set_network(&mut self) -> Result<(), AppError> {
        self.printer.set_network(self.state.network_id()?);
        Ok(())
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
                    sign_type: SignTxType::None,
                    tx_packet_count: 0,
                    tx_size: 0,
                    path: Bip32Path::new(0),
                    hasher: Hasher::new(),
                },
                extractor: InstructionExtractor::new(),
                printer: InstructionPrinter::new(NetworkId::LocalNet, LedgerTTY::new()),
            },
            show_digest: false,
        }
    }

    pub fn reset(&mut self) {
        self.processor.reset();
        self.decoder.reset();
        self.processor.set_tty(LedgerTTY::new());
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
            self.show_digest = comm.get_p1() == 1;
        } else {
            self.decode_tx_intent(comm.get_data()?, class)?;
        }

        if class == CommandClass::LastData {
            self.finalize_sign_tx(tx_type)
        } else {
            Ok(SignOutcome::SendNextPacket)
        }
    }

    fn finalize_sign_tx(&mut self, tx_type: SignTxType) -> Result<SignOutcome, AppError> {
        let digest = self.processor.state.finalize()?;

        if self.show_digest {
            self.processor.display_hex_string(digest.as_bytes());
        }

        if !ui::Validator::new("Sign Intent?").ask() {
            return Ok(SignOutcome::SigningRejected);
        }

        self.processor.sign_tx(tx_type, digest)
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
