use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::KeyPair25519;
use crate::crypto::secp256k1::KeyPairSecp256k1;
use crate::tx_sign_state::SignOutcome::Signature;
use nanos_sdk::bindings::{
    cx_err_t, cx_hash_sha256, cx_md_t, size_t, CX_LAST, CX_RND_RFC6979, CX_SHA256,
};
use nanos_sdk::io::Comm;
use nanos_ui::ui;
use sbor::decoder_error::DecoderError;
use sbor::instruction_extractor::{ExtractorEvent, InstructionExtractor, InstructionHandler};
use sbor::sbor_decoder::{DecodingOutcome, SborDecoder};
use sbor::sbor_notifications::SborEvent;

const SIGNATURE_SIZE: usize = 32;

#[repr(u8)]
#[derive(Eq, PartialEq, Copy, Clone)]
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
    intermediate_hash: [u8; 32],
    final_hash: [u8; 32],
}

extern "C" {
    pub fn cx_ecdsa_sign_no_throw(
        pvkey: *const u8,
        mode: u32,
        hashID: cx_md_t,
        hash: *const u8,
        hash_len: size_t,
        sig: *mut u8,
        sig_len: *mut size_t,
        info: *mut u32,
    ) -> cx_err_t;

    pub fn cx_eddsa_sign_no_throw(
        pvkey: *const u8,
        hashID: cx_md_t,
        hash: *const u8,
        hash_len: size_t,
        sig: *mut u8,
        sig_len: size_t,
    ) -> cx_err_t;
}

impl SignFlowState {
    fn process_data(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignTxType,
    ) -> Result<(), AppError> {
        self.validate(class, tx_type)?;

        if class == CommandClass::Regular {
            let path = Bip32Path::read(comm).and_then(|path| path.validate())?;
            self.start(tx_type, path);
        }

        let data = comm.get_data()?;
        self.update_counters(data.len());
        self.update_hash(data);

        if class == CommandClass::LastData {
            self.finalize_hash();
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.intermediate_hash.fill(0);
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

    //TODO: sign the hash, return signature to client
    fn sign_tx(&self, tx_type: SignTxType) -> Result<SignOutcome, AppError> {
        match tx_type {
            SignTxType::None => return Err(AppError::BadTxSignState),
            SignTxType::Ed25519 => KeyPair25519::derive(&self.path)
                .and_then(|keypair| keypair.sign(&self.intermediate_hash))
                .map(|signature| Signature(signature)),

            SignTxType::Secp256k1 => KeyPairSecp256k1::derive(&self.path)
                .and_then(|keypair| keypair.sign(&self.intermediate_hash))
                .map(|signature| Signature(signature)),
        }
    }

    fn handle_extractor_event(&mut self, event: ExtractorEvent) {
        // TODO collect data and display on instruction boundary
    }

    fn update_hash(&mut self, data: &[u8]) {
        unsafe {
            cx_hash_sha256(
                data.as_ptr(),
                data.len() as size_t,
                self.intermediate_hash.as_mut_ptr(),
                self.intermediate_hash.len() as size_t,
            );
        }
    }

    fn finalize_hash(&mut self) {
        unsafe {
            cx_hash_sha256(
                self.intermediate_hash.as_ptr(),
                self.intermediate_hash.len() as size_t,
                self.final_hash.as_mut_ptr(),
                self.final_hash.len() as size_t,
            );
        }
    }

    fn update_counters(&mut self, size: usize) {
        self.tx_size += size;
        self.tx_packet_count += 1;
    }
}

pub enum SignOutcome {
    SendNextPacket,
    SigningRejected,
    Signature([u8; SIGNATURE_SIZE]),
}

pub struct TxSignState {
    state: SignFlowState,
    decoder: SborDecoder,
    extractor: InstructionExtractor,
}

impl TxSignState {
    pub fn new() -> Self {
        Self {
            state: SignFlowState {
                sign_type: SignTxType::None,
                tx_packet_count: 0,
                tx_size: 0,
                intermediate_hash: [0; 32],
                final_hash: [0; 32],
                path: Bip32Path::new(0),
            },
            decoder: SborDecoder::new(),
            extractor: InstructionExtractor::new(),
        }
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
                self.state.reset(); // Ensure state is reset on error
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
        self.state.process_data(comm, class, tx_type)?;
        self.decode_tx_intent(comm.get_data()?, class)?;

        if class == CommandClass::LastData {
            self.finalize_sign_tx(tx_type)
        } else {
            Ok(SignOutcome::SendNextPacket)
        }
    }

    fn finalize_sign_tx(&mut self, tx_type: SignTxType) -> Result<SignOutcome, AppError> {
        //TODO: show hash to the user?

        // ask for confirmation, handle rejection
        if !ui::Validator::new("Sign Intent?").ask() {
            return Ok(SignOutcome::SigningRejected);
        }

        self.state.sign_tx(tx_type)
    }

    fn decode_tx_intent(&mut self, data: &[u8], class: CommandClass) -> Result<(), AppError> {
        let result = self.call_decoder(data);

        match result {
            Ok(outcome) => match outcome {
                DecodingOutcome::Done(size)
                    if size == self.state.tx_size && class == CommandClass::LastData =>
                {
                    Ok(())
                }
                DecodingOutcome::NeedMoreData(size)
                    if size == self.state.tx_size && class == CommandClass::Continuation =>
                {
                    Ok(())
                }
                _ => Err(AppError::BadTxSignLen),
            },
            Err(err) => Err(err.into()),
        }
    }

    fn call_decoder(&mut self, data: &[u8]) -> Result<DecodingOutcome, DecoderError> {
        let mut handler =
            InstructionHandler::new(SignFlowState::handle_extractor_event, &mut self.state);

        self.decoder.decode(&mut handler, data)
    }
}
