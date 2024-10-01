use crate::io::Comm;
use sbor::bech32::address::Address;
use sbor::bech32::network::NetworkId;
use sbor::digest::digest::Digest;
use sbor::digest::hash_calculator::{HashCalculatorMode, HashCalculator};
use sbor::instruction_extractor::InstructionExtractor;
use sbor::math::Decimal;
use sbor::print::fanout::Fanout;
use sbor::print::instruction_printer::InstructionPrinter;
use sbor::print::tty::TTY;
use sbor::print::tx_intent_type::TxIntentType;
use sbor::print::tx_summary_detector::{DetectedTxType, TxSummaryDetector};
use sbor::sbor_decoder::{SborEvent, SborEventHandler};

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::hash::Blake2bHasher;
use crate::sign::sign_mode::SignMode;
use crate::sign::sign_outcome::SignOutcome;
use crate::sign::signing_flow_state::SigningFlowState;

pub struct InstructionProcessor<T: Copy> {
    state: SigningFlowState,
    extractor: InstructionExtractor,
    printer: InstructionPrinter<T>,
    detector: TxSummaryDetector,
    calculator: HashCalculator<Blake2bHasher>,
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
            calculator: HashCalculator::<Blake2bHasher>::new(),
        }
    }

    pub fn set_intent_type(&mut self, intent_type: TxIntentType) {
        self.detector.set_intent_type(intent_type);
    }

    pub fn sign_message(
        &self,
        comm: &mut Comm,
        sign_mode: SignMode,
        message: &[u8],
    ) -> Result<SignOutcome, AppError> {
        self.state.sign_message(comm, sign_mode, message)
    }

    pub fn auth_digest(
        &mut self,
        challenge: &[u8],
        address: &[u8],
        origin: &[u8],
    ) -> Result<Digest, AppError> {
        self.calculator.auth_digest(challenge, address, origin)
    }

    pub fn tx_size(&self) -> usize {
        self.state.tx_size()
    }

    pub fn set_network(&mut self) -> Result<(), AppError> {
        match self.state.sign_mode() {
            SignMode::Ed25519Verbose
            | SignMode::Ed25519Summary
            | SignMode::AuthEd25519
            | SignMode::Ed25519PreAuthHash
            | SignMode::Ed25519Subintent => {
                let network_id = self.state.network_id()?;
                self.printer.set_network(network_id);
            }
            SignMode::Secp256k1Verbose | SignMode::Secp256k1Summary | SignMode::AuthSecp256k1 => {
                self.printer.set_network(NetworkId::OlympiaMainNet);
            }
        };
        Ok(())
    }

    pub fn set_show_instructions(&mut self) {
        match self.state.sign_mode() {
            SignMode::Secp256k1Summary
            | SignMode::Ed25519Summary
            | SignMode::AuthEd25519
            | SignMode::AuthSecp256k1
            | SignMode::Ed25519PreAuthHash
            | SignMode::Ed25519Subintent => {
                self.printer.set_show_instructions(false);
            }
            SignMode::Secp256k1Verbose | SignMode::Ed25519Verbose => {
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
        sign_mode: SignMode,
    ) -> Result<(), AppError> {
        // Prevent excessive optimization which causes stack overflow on Nano S
        core::intrinsics::black_box(match class {
            CommandClass::Regular => {
                self.state.init_sign(comm, sign_mode)?;

                let hash_mode = match sign_mode {
                    SignMode::Ed25519PreAuthHash | SignMode::Ed25519Subintent => {
                        HashCalculatorMode::Subintent
                    }
                    _ => HashCalculatorMode::Transaction,
                };

                self.calculator.start(hash_mode)
            }
            CommandClass::Continuation | CommandClass::LastData => {
                self.state.continue_sign(comm, class, sign_mode)
            }
        })
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
