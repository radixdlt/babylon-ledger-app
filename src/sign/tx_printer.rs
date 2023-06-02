use crate::sign::tx_intent_type::TxIntentType;
use sbor::bech32::network::NetworkId;
use sbor::instruction_extractor::{ExtractorEvent, InstructionHandler};
use sbor::math::Decimal;
use sbor::print::tty::TTY;
use sbor::tx_features::{TxFeatures, TxType};

pub struct TransactionPrinter<T: Copy> {
    tty: TTY<T>,
    network_id: NetworkId,
    found_fee: Option<Decimal>,
    found_features: TxFeatures,
    intent_type: TxIntentType,
}

impl<T: Copy> TransactionPrinter<T> {
    pub fn new(network_id: NetworkId, tty: TTY<T>) -> Self {
        Self {
            tty,
            network_id,
            found_fee: None,
            found_features: TxFeatures::new(),
            intent_type: TxIntentType::General,
        }
    }

    pub fn set_intent_type(&mut self, intent_type: TxIntentType) {
        self.intent_type = intent_type;
    }

    pub fn reset(&mut self) {
        self.found_fee = None;
        self.found_features.reset();
        self.intent_type = TxIntentType::General;
        self.network_id = NetworkId::LocalNet;
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.network_id = network_id;
    }
}

impl<T: Copy> InstructionHandler for TransactionPrinter<T> {
    fn handle(&mut self, event: ExtractorEvent) {
        // match event {
        //     ExtractorEvent::InstructionStart(info, count, total) => {
        //         self.start_instruction(info, count, total)
        //     }
        //     ExtractorEvent::ParameterStart(event, count, ..) => self.parameter_start(event, count),
        //     ExtractorEvent::ParameterData(data) => self.parameter_data(data),
        //     ExtractorEvent::ParameterEnd(event, ..) => self.parameter_end(event),
        //     ExtractorEvent::InstructionEnd => self.instruction_end(),
        //     // Error conditions
        //     ExtractorEvent::UnknownInstruction(..)
        //     | ExtractorEvent::InvalidEventSequence
        //     | ExtractorEvent::UnknownParameterType(..) => self.handle_error(),
        // };
    }
}
