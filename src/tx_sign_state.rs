use crate::app_error::AppError;
use crate::command_class::CommandClass;
use nanos_sdk::io::Comm;
use sbor::decoder_error::DecoderError;
use sbor::instruction_extractor::{ExtractorEvent, InstructionExtractor};
use sbor::sbor_decoder::{DecodingOutcome, SborDecoder};
use sbor::sbor_notifications::SborEvent;

#[repr(u8)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SignTxType {
    None,
    Ed25519,
    Secp256k1,
}

struct DecodingState {
    sign_type: SignTxType,
    tx_packet_count: u32,
    tx_size: usize,
    intermediate_hash: [u8; 64],
}

impl DecodingState {
    fn do_process(
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
        // todo!()
    }

    fn finalize(&self) -> Result<(), AppError> {
        // Finalize hash, check parser state, display it to user and ask confirmation
        // todo!()
        Ok(())
    }

    fn process_data(&mut self, data: &[u8]) -> Result<(), AppError> {
        self.update_counters(data.len());
        self.update_hash(data);
        // Add packet to hash, parse and display instructions to user, update counters
        // todo!()
        Ok(())
    }

    fn update_counters(&mut self, size: usize) {
        self.tx_size += size;
        self.tx_packet_count += 1;
    }

    fn update_hash(&mut self, data: &[u8]) {
        todo!()
    }
}

pub struct TxSignState {
    state: DecodingState,
    decoder: SborDecoder,
    extractor: InstructionExtractor,
}

impl TxSignState {
    pub fn new() -> Self {
        Self {
            state: DecodingState {
                sign_type: SignTxType::None,
                tx_packet_count: 0,
                tx_size: 0,
                intermediate_hash: [0; 64],
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
                self.state.reset();
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

        self.process_data(comm.get_data()?, class)?;

        if class == CommandClass::LastData {
            self.state.finalize()
        } else {
            Ok(())
        }
    }

    fn process_data(&mut self, data: &[u8], class: CommandClass) -> Result<(), AppError> {
        let mut unused : u32 = 0;
        let result = self.decoder.decode(
            |_: &mut u32, event: SborEvent| {
                self.extractor.handle_event(DecodingState::handle_extractor_event, &mut self.state, event);
            }, &mut unused, data);

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
}
