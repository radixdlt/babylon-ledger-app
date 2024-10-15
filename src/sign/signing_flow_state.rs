use crate::io::Comm;
use sbor::bech32::network::NetworkId;

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::KeyPair25519;
use crate::crypto::secp256k1::KeyPairSecp256k1;
use crate::sign::sign_mode::SignMode;
use crate::sign::sign_outcome::SignOutcome;

#[repr(align(4))]
pub struct SigningFlowState {
    sign_mode: SignMode,
    tx_packet_count: u32,
    tx_size: usize,
    path: Bip32Path,
}

impl SigningFlowState {
    pub fn new() -> Self {
        Self {
            sign_mode: SignMode::TxEd25519Verbose,
            tx_packet_count: 0,
            tx_size: 0,
            path: Bip32Path::new(0),
        }
    }

    pub fn continue_sign(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        sign_mode: SignMode,
    ) -> Result<(), AppError> {
        // Prevent excessive optimization which causes stack overflow on Nano S
        core::intrinsics::black_box(self.validate(class, sign_mode))?;

        // Prevent excessive optimization which causes stack overflow on Nano S
        let data = core::intrinsics::black_box(comm.get_data())?;
        self.update_counters(data.len());

        Ok(())
    }

    pub fn init_sign(&mut self, comm: &mut Comm, sign_mode: SignMode) -> Result<(), AppError> {
        let path = match sign_mode {
            SignMode::TxEd25519Verbose
            | SignMode::TxEd25519Summary
            | SignMode::AuthEd25519
            | SignMode::PreAuthHashEd25519 => Bip32Path::read_cap26(comm),
            SignMode::TxSecp256k1Verbose
            | SignMode::TxSecp256k1Summary
            | SignMode::AuthSecp256k1
            | SignMode::PreAuthHashSecp256k1 => Bip32Path::read_olympia(comm),
        }?;

        self.start(sign_mode, path);
        self.update_counters(0); // First packet contains no data
        Ok(())
    }

    #[inline(always)]
    pub fn network_id(&mut self) -> Result<NetworkId, AppError> {
        self.path.network_id()
    }

    #[inline(always)]
    pub fn tx_size(&self) -> usize {
        self.tx_size
    }

    #[inline(always)]
    pub fn sign_mode(&self) -> SignMode {
        self.sign_mode
    }

    pub fn reset(&mut self) {
        self.tx_packet_count = 0;
        self.tx_size = 0;
        self.sign_mode = SignMode::TxEd25519Summary;
        self.path = Bip32Path::new(0);
    }

    fn start(&mut self, sign_mode: SignMode, path: Bip32Path) {
        self.sign_mode = sign_mode;
        self.path = path;
    }

    #[inline(always)]
    fn sign_started(&self) -> bool {
        self.tx_packet_count != 0
    }

    fn validate(&self, class: CommandClass, sign_mode: SignMode) -> Result<(), AppError> {
        if self.sign_started() {
            self.validate_intermediate(class, sign_mode)
        } else {
            self.validate_initial(class)
        }
    }

    fn validate_intermediate(
        &self,
        class: CommandClass,
        sign_mode: SignMode,
    ) -> Result<(), AppError> {
        if self.sign_mode != sign_mode {
            return Err(AppError::BadTxSignType);
        }

        match class {
            CommandClass::Continuation | CommandClass::LastData => Ok(()),
            _ => Err(AppError::BadTxSignSequence),
        }
    }

    fn validate_initial(&self, class: CommandClass) -> Result<(), AppError> {
        if class != CommandClass::Regular {
            return Err(AppError::BadTxSignSequence);
        }

        Ok(())
    }

    pub fn sign_message(
        &self,
        comm: &mut Comm,
        sign_mode: SignMode,
        message: &[u8],
    ) -> Result<SignOutcome, AppError> {
        match sign_mode {
            SignMode::TxEd25519Verbose
            | SignMode::TxEd25519Summary
            | SignMode::AuthEd25519
            | SignMode::PreAuthHashEd25519 => {
                KeyPair25519::derive(&self.path).and_then(|keypair| keypair.sign(comm, message))
            }

            SignMode::TxSecp256k1Verbose
            | SignMode::TxSecp256k1Summary
            | SignMode::AuthSecp256k1
            | SignMode::PreAuthHashSecp256k1 => {
                KeyPairSecp256k1::derive(&self.path).and_then(|keypair| keypair.sign(comm, message))
            }
        }
    }

    fn update_counters(&mut self, size: usize) {
        self.tx_size += size;
        self.tx_packet_count += 1;
    }
}
