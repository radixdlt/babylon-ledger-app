use nanos_sdk::io::Comm;
use sbor::bech32::network::NetworkId;
use sbor::digest::digest::Digest;

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::bip32::Bip32Path;
use crate::crypto::ed25519::KeyPair25519;
use crate::crypto::secp256k1::KeyPairSecp256k1;
use crate::sign::sign_outcome::SignOutcome;
use crate::sign::sign_type::SignType;

#[repr(align(4))]
pub struct SigningFlowState {
    sign_type: SignType,
    tx_packet_count: u32,
    tx_size: usize,
    path: Bip32Path,
}

impl SigningFlowState {
    pub fn new() -> Self {
        Self {
            sign_type: SignType::Ed25519,
            tx_packet_count: 0,
            tx_size: 0,
            path: Bip32Path::new(0),
        }
    }

    pub fn continue_sign(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignType,
    ) -> Result<(), AppError> {
        self.validate(class, tx_type)?;
        let data = comm.get_data()?;
        self.update_counters(data.len());

        Ok(())
    }

    pub fn init_sign(&mut self, comm: &mut Comm, tx_type: SignType) -> Result<(), AppError> {
        let path = match tx_type {
            SignType::Ed25519 | SignType::Ed25519Summary | SignType::AuthEd25519 => {
                Bip32Path::read_cap26(comm)
            }
            SignType::Secp256k1 | SignType::Secp256k1Summary | SignType::AuthSecp256k1 => {
                Bip32Path::read_olympia(comm)
            }
        }?;

        self.start(tx_type, path);
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
    pub fn sign_type(&self) -> SignType {
        self.sign_type
    }

    pub fn reset(&mut self) {
        self.tx_packet_count = 0;
        self.tx_size = 0;
        self.sign_type = SignType::Ed25519;
        self.path = Bip32Path::new(0);
    }

    fn start(&mut self, sign_type: SignType, path: Bip32Path) {
        self.sign_type = sign_type;
        self.path = path;
    }

    #[inline(always)]
    fn sign_started(&self) -> bool {
        self.tx_packet_count != 0
    }

    fn validate(&self, class: CommandClass, sign_type: SignType) -> Result<(), AppError> {
        if self.sign_started() {
            self.validate_intermediate(class, sign_type)
        } else {
            self.validate_initial(class)
        }
    }

    fn validate_intermediate(
        &self,
        class: CommandClass,
        sign_type: SignType,
    ) -> Result<(), AppError> {
        if self.sign_type != sign_type {
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

    pub fn sign_tx(
        &self,
        comm: &mut Comm,
        tx_type: SignType,
        digest: &Digest,
    ) -> Result<SignOutcome, AppError> {
        match tx_type {
            SignType::Ed25519 | SignType::Ed25519Summary | SignType::AuthEd25519 => {
                KeyPair25519::derive(&self.path)
                    .and_then(|keypair| keypair.sign(comm, digest.as_bytes()))
            }

            SignType::Secp256k1 | SignType::Secp256k1Summary | SignType::AuthSecp256k1 => {
                KeyPairSecp256k1::derive(&self.path)
                    .and_then(|keypair| keypair.sign(comm, digest.as_bytes()))
            }
        }
    }

    fn update_counters(&mut self, size: usize) {
        self.tx_size += size;
        self.tx_packet_count += 1;
    }
}
