use core::intrinsics::size_of;
use nanos_sdk::io::Comm;
use sbor::instruction_extractor::{ExtractorEvent, InstructionExtractor};
use sbor::sbor_decoder::SborDecoder;
use sbor::sbor_notifications::SborEvent;
use crate::app_error::AppError;
use crate::command_class::CommandClass;

#[repr(u8)]
#[derive(Eq, PartialEq)]
pub enum SignTxType {
    None,
    Ed25519,
    Secp256k1,
}

pub struct TxSignState {
    sign_type: SignTxType,
    tx_packet_count: u32,
    tx_size: u32,
    intermediate_hash: [u8;64],
    decoder: SborDecoder<FnMut(SborEvent)>,
    extractor: InstructionExtractor<FnMut(ExtractorEvent)>,
}

impl TxSignState {
    pub fn new() -> Self {
        Self {
            sign_type: SignTxType::None,
            tx_packet_count: 0,
            tx_size: 0,
            intermediate_hash: [0; 64],
            decoder: SborDecoder::new(|evt: SborEvent| {
            }),
            extractor: InstructionExtractor::new(|evt: ExtractorEvent| {
            })
        }
    }

    pub fn start_sign(&mut self, sign_type: SignTxType) {
        self.sign_type = sign_type;
    }

    pub fn process_packet(&mut self, comm: &Comm) {
        todo!()
    }

    pub fn end_of_tx_data(&mut self) {
        self.intermediate_hash.fill(0);
        self.tx_packet_count = 0;
        self.tx_size = 0;
        self.sign_type = SignTxType::None;
    }

    pub fn sign_started(&self) -> bool {
        self.sign_type != SignTxType::None && self.tx_packet_count != 0
    }

    pub fn validate(&self, class: CommandClass, sign_type: SignTxType) -> Result<(), AppError> {
        if self.sign_started() {
            self.validate_intermediate(class, sign_type)
        } else {
            self.validate_initial(class)
        }
    }

    fn validate_intermediate(&self, class: CommandClass, sign_type: SignTxType) -> Result<(), AppError> {
        if self.sign_type != sign_type {
            return Err(AppError::BadTxSignState);
        }

        if class != CommandClass::Continuation {
            return Err(AppError::BadTxSignSequence);
        }

        Ok(())
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
}