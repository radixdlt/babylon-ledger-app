use crate::crypto::curves::Curve;
use crate::sign::decoding_mode::DecodingMode;
use sbor::digest::hash_calculator::HashCalculatorMode;

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum SignMode {
    TxEd25519Verbose,
    TxEd25519Summary,
    TxSecp256k1Verbose,
    TxSecp256k1Summary,
    AuthEd25519,
    AuthSecp256k1,
    PreAuthHashEd25519,
    PreAuthHashSecp256k1,
    PreAuthRawEd25519,
    PreAuthRawSecp256k1,
}

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum SignType {
    None,
    Verbose,
    Summary,
}

#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum ReviewType {
    Transaction,
    OwnershipProof,
    PreAuthHash,
    PreAuthRaw,
}

impl SignMode {
    pub fn requires_blind_signing(&self) -> bool {
        matches!(
            self,
            SignMode::PreAuthHashEd25519
                | SignMode::PreAuthRawEd25519
                | SignMode::PreAuthHashSecp256k1
                | SignMode::PreAuthRawSecp256k1
        )
    }
    pub fn curve(&self) -> Curve {
        match self {
            SignMode::TxEd25519Verbose
            | SignMode::TxEd25519Summary
            | SignMode::AuthEd25519
            | SignMode::PreAuthHashEd25519
            | SignMode::PreAuthRawEd25519 => Curve::Ed25519,
            SignMode::TxSecp256k1Verbose
            | SignMode::TxSecp256k1Summary
            | SignMode::AuthSecp256k1
            | SignMode::PreAuthHashSecp256k1
            | SignMode::PreAuthRawSecp256k1 => Curve::Secp256k1,
        }
    }

    pub fn sign_type(&self) -> SignType {
        match self {
            SignMode::TxEd25519Verbose | SignMode::TxSecp256k1Verbose => SignType::Verbose,
            SignMode::TxEd25519Summary | SignMode::TxSecp256k1Summary => SignType::Summary,
            _ => SignType::None,
        }
    }

    pub fn review_type(&self) -> ReviewType {
        match self {
            SignMode::TxEd25519Verbose
            | SignMode::TxSecp256k1Verbose
            | SignMode::TxEd25519Summary
            | SignMode::TxSecp256k1Summary => ReviewType::Transaction,
            SignMode::AuthEd25519 | SignMode::AuthSecp256k1 => ReviewType::OwnershipProof,
            SignMode::PreAuthHashEd25519 | SignMode::PreAuthHashSecp256k1 => {
                ReviewType::PreAuthHash
            }
            SignMode::PreAuthRawEd25519 | SignMode::PreAuthRawSecp256k1 => ReviewType::PreAuthRaw,
        }
    }

    pub fn shows_instructions(&self) -> bool {
        match self {
            SignMode::TxSecp256k1Summary
            | SignMode::TxEd25519Summary
            | SignMode::AuthEd25519
            | SignMode::AuthSecp256k1
            | SignMode::PreAuthHashEd25519
            | SignMode::PreAuthHashSecp256k1
            | SignMode::PreAuthRawEd25519
            | SignMode::PreAuthRawSecp256k1 => false,
            SignMode::TxSecp256k1Verbose | SignMode::TxEd25519Verbose => true,
        }
    }

    pub fn hash_mode(&self) -> HashCalculatorMode {
        match self {
            SignMode::PreAuthHashEd25519
            | SignMode::PreAuthHashSecp256k1
            | SignMode::PreAuthRawEd25519
            | SignMode::PreAuthRawSecp256k1 => HashCalculatorMode::PreAuth,
            _ => HashCalculatorMode::Transaction,
        }
    }

    pub fn decoding_mode(&self) -> DecodingMode {
        match self {
            SignMode::AuthEd25519 | SignMode::AuthSecp256k1 => DecodingMode::Auth,
            SignMode::PreAuthHashEd25519 | SignMode::PreAuthHashSecp256k1 => DecodingMode::PreAuth,
            SignMode::TxEd25519Verbose
            | SignMode::TxSecp256k1Verbose
            | SignMode::TxEd25519Summary
            | SignMode::TxSecp256k1Summary
            | SignMode::PreAuthRawEd25519
            | SignMode::PreAuthRawSecp256k1 => DecodingMode::Transaction,
        }
    }
}
