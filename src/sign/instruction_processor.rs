use nanos_sdk::io::Comm;
use sbor::bech32::network::NetworkId;
use sbor::instruction_extractor::InstructionExtractor;
use sbor::math::Decimal;
use sbor::print::fanout::Fanout;
use sbor::print::instruction_printer::InstructionPrinter;
use sbor::print::tty::TTY;
use sbor::print::tx_printer::{Address, DetectedTxType};
use sbor::sbor_decoder::{SborEvent, SborEventHandler};

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::hash::Digest;
use crate::sign::sign_outcome::SignOutcome;
use crate::sign::sign_type::SignType;
use crate::sign::signing_flow_state::SigningFlowState;
use sbor::print::tx_intent_type::TxIntentType;
use sbor::print::tx_printer::TxIntentPrinter;

pub struct InstructionProcessor<T: Copy> {
    state: SigningFlowState,
    extractor: InstructionExtractor,
    ins_printer: InstructionPrinter<T>,
    tx_printer: TxIntentPrinter,
}

impl<T: Copy> SborEventHandler for InstructionProcessor<T> {
    fn handle(&mut self, evt: SborEvent) {
        let mut fanout = Fanout::new(&mut self.ins_printer, &mut self.tx_printer);
        self.extractor.handle_event(&mut fanout, evt);
    }
}

impl<T: Copy> InstructionProcessor<T> {
    pub fn new(tty: TTY<T>) -> Self {
        Self {
            state: SigningFlowState::new(),
            extractor: InstructionExtractor::new(),
            ins_printer: InstructionPrinter::new(NetworkId::LocalNet, tty),
            tx_printer: TxIntentPrinter::new(),
        }
    }

    pub fn set_intent_type(&mut self, intent_type: TxIntentType) {
        self.tx_printer.set_intent_type(intent_type);
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
        self.state.auth_digest(nonce, address, origin)
    }

    pub fn tx_size(&self) -> usize {
        self.state.tx_size()
    }

    pub fn set_network(&mut self) -> Result<(), AppError> {
        match self.state.sign_type() {
            SignType::Ed25519 | SignType::Ed25519Summary | SignType::AuthEd25519 => {
                let network_id = self.state.network_id()?;
                self.ins_printer.set_network(network_id);
            }
            SignType::Secp256k1 | SignType::Secp256k1Summary | SignType::AuthSecp256k1 => {
                self.ins_printer.set_network(NetworkId::OlympiaMainNet);
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
                self.ins_printer.set_show_instructions(false);
            }
            SignType::Secp256k1 | SignType::Ed25519 => {
                self.ins_printer.set_show_instructions(true);
            }
        };
    }

    pub fn set_tty(&mut self, tty: TTY<T>) {
        self.ins_printer.set_tty(tty);
    }

    pub fn reset(&mut self) {
        self.state.reset();
        self.extractor.reset();
        self.ins_printer.reset();
        self.tx_printer.reset();
    }

    pub fn finalize(&mut self) -> Result<Digest, AppError> {
        self.state.finalize()
    }

    pub fn process_sign(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignType,
    ) -> Result<(), AppError> {
        self.state.process_sign(comm, class, tx_type)
    }

    pub fn get_detected_tx_type(&self) -> DetectedTxType {
        self.tx_printer.get_detected_tx_type()
    }

    pub fn format_decimal(&mut self, value: &Decimal, suffix: &[u8]) -> &[u8] {
        self.ins_printer.format_decimal(value, suffix)
    }

    pub fn format_address(&mut self, address: &Address) -> &[u8] {
        self.ins_printer.format_address(address)
    }
}
