use nanos_sdk::io::Comm;
use sbor::bech32::network::NetworkId;
use sbor::instruction_extractor::InstructionExtractor;
use sbor::print::instruction_printer::InstructionPrinter;
use sbor::print::tty::TTY;
use sbor::sbor_decoder::{SborEvent, SborEventHandler};

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::hash::Digest;
use crate::ledger_display_io::LedgerTTY;
use crate::sign::flow_state::SigningFlowState;
use crate::sign::sign_outcome::SignOutcome;
use crate::sign::sign_type::SignType;

pub struct InstructionProcessor {
    state: SigningFlowState,
    extractor: InstructionExtractor,
    printer: InstructionPrinter,
}

impl SborEventHandler for InstructionProcessor {
    fn handle(&mut self, evt: SborEvent) {
        self.extractor.handle_event(&mut self.printer, evt);
    }
}

impl InstructionProcessor {
    pub fn new() -> Self {
        Self {
            state: SigningFlowState::new(),
            extractor: InstructionExtractor::new(),
            printer: InstructionPrinter::new(NetworkId::LocalNet, LedgerTTY::new_tty()),
        }
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
                self.printer.set_network(self.state.network_id()?)
            }
            SignType::Secp256k1 | SignType::Secp256k1Summary | SignType::AuthSecp256k1 => {
                self.printer.set_network(NetworkId::OlympiaMainNet)
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

    pub fn set_tty(&mut self, tty: TTY) {
        self.printer.set_tty(tty);
    }

    pub fn reset(&mut self) {
        self.state.reset();
        self.extractor.reset();
        self.printer.reset();
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
}
