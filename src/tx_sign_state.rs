use crate::app_error::AppError;
use crate::command_class::CommandClass;
use nanos_sdk::bindings::{cx_hash_sha256, size_t};
use nanos_sdk::io::Comm;
use sbor::decoder_error::DecoderError;
use sbor::instruction_extractor::{ExtractorEvent, InstructionExtractor, InstructionHandler};
use sbor::sbor_decoder::{DecodingOutcome, SborDecoder};
use sbor::sbor_notifications::SborEvent;

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
    intermediate_hash: [u8; 32],
    final_hash: [u8; 32],
}

impl SignFlowState {
    fn process_data(
        &mut self,
        comm: &Comm,
        class: CommandClass,
        tx_type: SignTxType,
    ) -> Result<(), AppError> {
        self.validate(class, tx_type)?;

        let data = comm.get_data()?;

        if class == CommandClass::Regular {
            self.start(tx_type);
        }

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
    }

    fn start(&mut self, sign_type: SignTxType) {
        self.reset();
        self.sign_type = sign_type;
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
            },
            decoder: SborDecoder::new(),
            extractor: InstructionExtractor::new(),
        }
    }

    pub fn process_request(
        &mut self,
        comm: &Comm,
        class: CommandClass,
        tx_type: SignTxType,
    ) -> Result<(), AppError> {
        let result = self.do_process(comm, class, tx_type);

        match result {
            Ok(()) => result,
            Err(_) => {
                self.state.reset(); // Ensure state is reset on error
                result
            }
        }
    }

    fn do_process(
        &mut self,
        comm: &Comm,
        class: CommandClass,
        tx_type: SignTxType,
    ) -> Result<(), AppError> {
        self.state.validate(class, tx_type)?;

        if class == CommandClass::Regular {
            self.state.start(tx_type);
        }

        self.state.process_data(comm, class, tx_type)?;
        self.decode_tx_intent(comm.get_data()?, class)?;

        if class == CommandClass::LastData {
            //TODO: show hash to the user, process confirmation/rejection
            Ok(())
        } else {
            Ok(())
        }
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
