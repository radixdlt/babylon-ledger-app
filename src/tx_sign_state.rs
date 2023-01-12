use crate::app_error::AppError;
use crate::command_class::CommandClass;
use nanos_sdk::io::Comm;
use sbor::instruction_extractor::{ExtractorEvent, InstructionExtractor};
use sbor::sbor_decoder::SborDecoder;
use sbor::sbor_notifications::SborEvent;

#[repr(u8)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SignTxType {
    None,
    Ed25519,
    Secp256k1,
}

pub struct TxSignState {
    sign_type: SignTxType,
    tx_packet_count: u32,
    tx_size: u32,
    intermediate_hash: [u8; 64],
    decoder: SborDecoder<TxSignState>,
    extractor: InstructionExtractor<TxSignState>,
}

impl TxSignState {
    pub fn new() -> Self {
        Self {
            sign_type: SignTxType::None,
            tx_packet_count: 0,
            tx_size: 0,
            intermediate_hash: [0; 64],
            decoder: SborDecoder::new(TxSignState::handle_decoder_event),
            extractor: InstructionExtractor::new(TxSignState::handle_extractor_event),
        }
    }

    pub fn process_request(
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

        self.process_data(data)?;

        if class == CommandClass::LastData {
            self.finalize()
        } else {
            Ok(())
        }
    }

    fn start(&mut self, sign_type: SignTxType) {
        self.reset();
        self.sign_type = sign_type;
    }

    fn reset(&mut self) {
        self.intermediate_hash.fill(0);
        self.tx_packet_count = 0;
        self.tx_size = 0;
        self.sign_type = SignTxType::None;
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

    fn finalize(&self) -> Result<(), AppError> {
        // Finalize hash, display it to user and ask confirmation
        todo!()
    }

    fn process_data(&self, data: &[u8]) -> Result<(), AppError> {
        // Add packet to hash, parse and display instructions to user, update counters
        todo!()
    }

    fn handle_decoder_event(&mut self, event: SborEvent) {
        todo!()
    }

    fn handle_extractor_event(&mut self, event: ExtractorEvent) {
        todo!()
    }
}
