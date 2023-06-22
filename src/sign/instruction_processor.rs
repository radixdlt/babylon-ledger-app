use nanos_sdk::io::Comm;
use sbor::bech32::network::NetworkId;
use sbor::digest::digest::Digest;
use sbor::digest::tx_hash_calculator::TxHashCalculator;
use sbor::instruction_extractor::InstructionExtractor;
use sbor::math::Decimal;
use sbor::print::fanout::Fanout;
use sbor::print::instruction_printer::InstructionPrinter;
use sbor::print::tty::TTY;
use sbor::print::tx_intent_type::TxIntentType;
use sbor::print::tx_summary_detector::{Address, DetectedTxType, TxSummaryDetector};
use sbor::sbor_decoder::{SborEvent, SborEventHandler};

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::hash::Blake2bHasher;
use crate::sign::sign_outcome::SignOutcome;
use crate::sign::sign_type::SignType;
use crate::sign::signing_flow_state::SigningFlowState;

pub struct InstructionProcessor<T: Copy> {
    state: SigningFlowState,
    extractor: InstructionExtractor,
    printer: InstructionPrinter<T>,
    detector: TxSummaryDetector,
    calculator: TxHashCalculator<Blake2bHasher>,
}

impl<T: Copy> SborEventHandler for InstructionProcessor<T> {
    fn handle(&mut self, evt: SborEvent) {
        let mut fanout = Fanout::new(&mut self.printer, &mut self.detector);
        self.extractor.handle_event(&mut fanout, evt);
        self.calculator.handle(evt);
    }
}

impl<T: Copy> InstructionProcessor<T> {
    pub fn new(tty: TTY<T>) -> Self {
        Self {
            state: SigningFlowState::new(),
            extractor: InstructionExtractor::new(),
            printer: InstructionPrinter::new(NetworkId::LocalNet, tty),
            detector: TxSummaryDetector::new(),
            calculator: TxHashCalculator::<Blake2bHasher>::new(),
        }
    }

    pub fn set_intent_type(&mut self, intent_type: TxIntentType) {
        self.detector.set_intent_type(intent_type);
    }

    pub fn sign_tx(&self, tx_type: SignType, digest: Digest) -> Result<SignOutcome, AppError> {
        self.state.sign_tx(tx_type, digest)
    }

    pub fn auth_digest(
        &mut self,
        nonce: &[u8],
        address: &[u8],
        origin: &[u8],
    ) -> Result<Digest, AppError> {
        self.calculator.auth_digest(nonce, address, origin)
    }

    pub fn tx_size(&self) -> usize {
        self.state.tx_size()
    }

    pub fn set_network(&mut self) -> Result<(), AppError> {
        match self.state.sign_type() {
            SignType::Ed25519 | SignType::Ed25519Summary | SignType::AuthEd25519 => {
                let network_id = self.state.network_id()?;
                self.printer.set_network(network_id);
            }
            SignType::Secp256k1 | SignType::Secp256k1Summary | SignType::AuthSecp256k1 => {
                self.printer.set_network(NetworkId::OlympiaMainNet);
            }
        };
        Ok(())
    }

    pub fn set_show_instructions(&mut self) {
        match self.state.sign_type() {
            SignType::Secp256k1Summary
            | SignType::Ed25519Summary
            | SignType::AuthEd25519
            | SignType::AuthSecp256k1 => {
                self.printer.set_show_instructions(false);
            }
            SignType::Secp256k1 | SignType::Ed25519 => {
                self.printer.set_show_instructions(true);
            }
        };
    }

    pub fn set_tty(&mut self, tty: TTY<T>) {
        self.printer.set_tty(tty);
    }

    pub fn reset(&mut self) {
        self.state.reset();
        self.calculator.reset();
        self.extractor.reset();
        self.printer.reset();
        self.detector.reset();
    }

    pub fn finalize(&mut self) -> Result<Digest, AppError> {
        self.calculator.finalize()
    }

    pub fn process_sign(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignType,
    ) -> Result<(), AppError> {
        match class {
            CommandClass::Regular => {
                self.state.init_sign(comm, tx_type)?;
                self.calculator.start()
            }
            CommandClass::Continuation | CommandClass::LastData => {
                self.state.continue_sign(comm, class, tx_type)
            }
        }
    }

    pub fn get_detected_tx_type(&self) -> DetectedTxType {
        self.detector.get_detected_tx_type()
    }

    pub fn format_decimal(&mut self, value: &Decimal, suffix: &[u8]) -> &[u8] {
        self.printer.format_decimal(value, suffix)
    }

    pub fn format_address(&mut self, address: &Address) -> &[u8] {
        self.printer.format_address(address)
    }
}
