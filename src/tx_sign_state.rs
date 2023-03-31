use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::{KeyPair25519, ED25519_SIGNATURE_LEN};
use crate::crypto::hash::{Digest, HashType, Hasher};
use crate::crypto::secp256k1::{KeyPairSecp256k1, SECP256K1_SIGNATURE_LEN};
use core::cmp::max;

use nanos_sdk::io::Comm;
use nanos_ui::ui;
use sbor::bech32::network::NetworkId;
use sbor::decoder_error::DecoderError;
use sbor::instruction_extractor::InstructionExtractor;
use sbor::print::instruction_printer::InstructionPrinter;
use sbor::print::tty::TTY;
use sbor::sbor_decoder::*;

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
        self.validate(class, tx_type)?;

        match class {
            CommandClass::Regular => {
                let path = Bip32Path::read(comm).and_then(|path| path.validate())?;
                self.start(tx_type, path);
            }

            CommandClass::Continuation | CommandClass::LastData => {
                let data = comm.get_data()?;
                self.update_counters(data.len());
                self.hasher.update(data)?;
            }

            CommandClass::Unknown => {
                return Err(AppError::BadTxSignSequence);
            }
        }
        Ok(())
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

    fn start(&mut self, sign_type: SignTxType, path: Bip32Path) {
        self.reset();
        self.sign_type = sign_type;
        self.path = path;
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
            return Err(AppError::BadTxSignState);
        }

        match class {
            CommandClass::Continuation | CommandClass::LastData => Ok(()),
            _ => return Err(AppError::BadTxSignSequence),
        }
    }

    fn validate_initial(&self, class: CommandClass) -> Result<(), AppError> {
        if self.sign_type != SignTxType::None {
            return Err(AppError::BadTxSignState);
        }

        if class != CommandClass::Regular {
            return Err(AppError::BadTxSignSequence);
        }

        Ok(())
    }

    fn sign_tx(&self, tx_type: SignTxType, digest: Digest) -> Result<SignOutcome, AppError> {
        self.validate_digest(tx_type, digest)?;

        match tx_type {
            SignTxType::None => return Err(AppError::BadTxSignState),
            SignTxType::Ed25519 => KeyPair25519::derive(&self.path)
                .and_then(|keypair| keypair.sign(digest.as_bytes()))
                .map(|signature| {
                    let mut full_signature = [0u8; MAX_SIGNATURE_SIZE];
                    full_signature.copy_from_slice(&signature);

                    SignOutcome::Signature {
                        len: signature.len() as u8,
                        signature: full_signature,
                    }
                }),

            SignTxType::Secp256k1 => KeyPairSecp256k1::derive(&self.path)
                .and_then(|keypair| keypair.sign(digest.as_bytes()))
                .map(|signature| SignOutcome::Signature {
                    len: signature.len() as u8,
                    signature,
                }),
        }
    }

    fn update_counters(&mut self, size: usize) {
        self.tx_size += size;
        self.tx_packet_count += 1;
    }

    fn validate_digest(&self, tx_type: SignTxType, digest: Digest) -> Result<(), AppError> {
        match (tx_type, digest.hash_type()) {
            (SignTxType::Ed25519, HashType::SHA512) => Ok(()),
            (SignTxType::Secp256k1, HashType::DoubleSHA256) => Ok(()),
            _ => Err(AppError::BadTxSignState),
        }
    }
}

const MAX_SIGNATURE_SIZE: usize = max(ED25519_SIGNATURE_LEN, SECP256K1_SIGNATURE_LEN);

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum SignOutcome {
    SendNextPacket,
    SigningRejected,
    Signature {
        len: u8,
        signature: [u8; MAX_SIGNATURE_SIZE],
    },
}

struct InstructionProcessor<'a> {
    state: SignFlowState,
    extractor: InstructionExtractor,
    printer: InstructionPrinter<'a>,
}

impl<'a> InstructionProcessor<'a> {
    pub fn sign_tx(&self, tx_type: SignTxType, digest: Digest) -> Result<SignOutcome, AppError> {
        self.state.sign_tx(tx_type, digest)
    }

    pub fn tx_size(&self) -> usize {
        self.state.tx_size
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.printer.set_network(network_id);
    }

    pub fn process_data(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignTxType,
    ) -> Result<(), AppError> {
        self.state.process_data(comm, class, tx_type)
    }

    pub fn reset(&mut self) {
        self.state.reset();
    }

    pub fn finalize(&mut self) -> Result<Digest, AppError> {
        self.state.finalize()
    }
}

pub struct TxSignState<'a> {
    decoder: SborDecoder,
    processor: InstructionProcessor<'a>,
}

impl SborEventHandler for InstructionProcessor<'_> {
    fn handle(&mut self, evt: SborEvent) {
        self.extractor.handle_event(&mut self.printer, evt);
    }
}

impl<'a> TxSignState<'a> {
    pub fn new(tty: &'a mut dyn TTY) -> Self {
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
                printer: InstructionPrinter::new(tty, NetworkId::LocalNet),
            },
        }
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.processor.set_network(network_id);
    }

    pub fn process_request(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignTxType,
    ) -> Result<SignOutcome, AppError> {
        let result = self.do_process(comm, class, tx_type);

        match result {
            Ok(_) => result,
            Err(_) => {
                self.processor.reset(); // Ensure state is reset on error
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
        self.processor.process_data(comm, class, tx_type)?;
        self.decode_tx_intent(comm.get_data()?, class)?;

        if class == CommandClass::LastData {
            self.finalize_sign_tx(tx_type)
        } else {
            Ok(SignOutcome::SendNextPacket)
        }
    }

    fn finalize_sign_tx(&mut self, tx_type: SignTxType) -> Result<SignOutcome, AppError> {
        //TODO: show hash to the user?
        let digest = self.processor.state.finalize()?;

        // ask for confirmation, handle rejection
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
