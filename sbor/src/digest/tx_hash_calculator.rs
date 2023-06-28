use core::result::Result;

use crate::digest::digest::Digest;
use crate::digest::digester::Digester;
use crate::sbor_decoder::SborEvent;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum TxHashPhase {
    Start,
    Header,
    Instructions,
    Blobs,
    SingleBlob,
    SingleBlobLen,
    SingleBlobData,
    Attachments,
    DecodingError,
    HashingError,
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum HashCommitPhase {
    None,
    CommitRegular,
    CommitBlob,
}

pub struct TxHashCalculator<T: Digester> {
    work_digester: T,
    blob_digester: T,
    output_digester: T,
    phase: TxHashPhase,
    commit_phase: HashCommitPhase,
}

impl<T: Digester> TxHashCalculator<T> {
    const PAYLOAD_PREFIX: u8 = 0x54;
    const VERSION_DISCRIMINATOR: u8 = 0x01;
    const INITIAL_VECTOR: [u8; 2] = [Self::PAYLOAD_PREFIX, Self::VERSION_DISCRIMINATOR];

    pub fn new() -> Self {
        Self {
            work_digester: T::new(),
            blob_digester: T::new(),
            output_digester: T::new(),
            phase: TxHashPhase::Start,
            commit_phase: HashCommitPhase::None,
        }
    }

    #[inline(always)]
    pub fn auth_digest(
        &mut self,
        nonce: &[u8],
        address: &[u8],
        origin: &[u8],
    ) -> Result<Digest, T::Error> {
        self.reset();
        self.work_digester.init()?;
        self.work_digester.update(nonce)?;
        self.work_digester.update(address)?;
        self.work_digester.update(origin)?;
        self.work_digester.finalize()
    }

    #[inline(always)]
    pub fn finalize(&mut self) -> Result<Digest, T::Error> {
        self.output_digester.finalize()
    }

    pub fn reset(&mut self) {
        self.work_digester.reset();
        self.blob_digester.reset();
        self.output_digester.reset();
        self.phase = TxHashPhase::Start;
        self.commit_phase = HashCommitPhase::None;
    }

    pub fn start(&mut self) -> Result<(), T::Error> {
        self.work_digester.init()?;
        self.blob_digester.init()?;
        self.output_digester.init()?;
        self.output_digester.update(&Self::INITIAL_VECTOR)
    }

    pub fn handle(&mut self, event: SborEvent) {
        match event {
            SborEvent::InputByte(byte) => self.put_byte(byte),
            SborEvent::Start {
                type_id: _,
                nesting_level,
                ..
            } => self.process_start(nesting_level),
            SborEvent::End {
                type_id: _,
                nesting_level,
                ..
            } => self.process_end(nesting_level),
            SborEvent::Len(_) if self.phase == TxHashPhase::SingleBlob => {
                self.phase = TxHashPhase::SingleBlobLen
            }
            _ => {}
        }
    }

    fn put_byte(&mut self, byte: u8) {
        match self.phase {
            TxHashPhase::Start
            | TxHashPhase::DecodingError
            | TxHashPhase::HashingError
            | TxHashPhase::Blobs
            | TxHashPhase::SingleBlob => return,
            TxHashPhase::SingleBlobLen => {
                self.phase = TxHashPhase::SingleBlobData;
                return;
            }
            _ => {}
        };

        let digester = if self.phase == TxHashPhase::SingleBlobData {
            &mut self.blob_digester
        } else {
            &mut self.work_digester
        };

        match digester.update(&[byte]) {
            Err(..) => self.phase = TxHashPhase::HashingError,
            _ => {}
        }

        match self.commit_phase {
            HashCommitPhase::None => {}
            HashCommitPhase::CommitRegular => self.finalize_and_push(),
            HashCommitPhase::CommitBlob => {
                self.finalize_and_push_blob();
                self.phase = TxHashPhase::Blobs;
            }
        }
        self.commit_phase = HashCommitPhase::None;
    }

    fn process_start(&mut self, nesting_level: u8) {
        match (self.phase, nesting_level) {
            (TxHashPhase::Start, 1) => self.phase = TxHashPhase::Header,
            (TxHashPhase::Header, 1) => self.phase = TxHashPhase::Instructions,
            (TxHashPhase::Instructions, 1) => self.phase = TxHashPhase::Blobs,
            (TxHashPhase::Blobs, 2) => self.phase = TxHashPhase::SingleBlob,
            (TxHashPhase::Blobs, 1) => {
                self.finalize_and_push();
                self.commit_phase = HashCommitPhase::None;
                self.phase = TxHashPhase::Attachments;
            }
            (TxHashPhase::Attachments, 1) => self.phase = TxHashPhase::DecodingError,
            (_, _) => {}
        }
    }

    fn process_end(&mut self, nesting_level: u8) {
        match (self.phase, nesting_level) {
            (TxHashPhase::Header, 1) => self.commit_phase = HashCommitPhase::CommitRegular,
            (TxHashPhase::Instructions, 1) => self.commit_phase = HashCommitPhase::CommitRegular,
            (TxHashPhase::Blobs, 1) => self.commit_phase = HashCommitPhase::CommitRegular,
            (TxHashPhase::SingleBlobData, 2) => self.commit_phase = HashCommitPhase::CommitBlob,
            (TxHashPhase::Attachments, 1) => self.commit_phase = HashCommitPhase::CommitRegular,
            (_, _) => {}
        }
    }

    fn finalize_and_push(&mut self) {
        match self.work_digester.finalize() {
            Ok(digest) => match self.output_digester.update(digest.as_bytes()) {
                Ok(_) => {}
                Err(_) => {
                    self.phase = TxHashPhase::HashingError;
                }
            },
            Err(_) => {
                self.phase = TxHashPhase::HashingError;
            }
        }

        match self.work_digester.init() {
            Ok(_) => {}
            Err(_) => {
                self.phase = TxHashPhase::HashingError;
            }
        }
    }

    fn finalize_and_push_blob(&mut self) {
        match self.blob_digester.finalize() {
            Ok(digest) => match self.work_digester.update(digest.as_bytes()) {
                Ok(_) => {}
                Err(_) => {
                    self.phase = TxHashPhase::HashingError;
                }
            },
            Err(_) => {
                self.phase = TxHashPhase::HashingError;
            }
        };

        match self.blob_digester.init() {
            Ok(_) => {}
            Err(_) => {
                self.phase = TxHashPhase::HashingError;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use blake2::digest::consts::U32;
    use blake2::digest::Digest as BlakeDigest;
    use blake2::Blake2b;

    use crate::digest::digest::Digest;
    use crate::digest::digester::Digester;
    use crate::digest::tx_hash_calculator::TxHashCalculator;
    use crate::sbor_decoder::{SborDecoder, SborEvent, SborEventHandler};
    use crate::tx_intent_test_data::tests::*;

    type Blake2b256 = Blake2b<U32>;

    #[derive(Copy, Clone, Debug)]
    enum HasherError {}

    struct TestDigester {
        hasher: Blake2b256,
    }

    impl Digester for TestDigester {
        type Error = HasherError;

        fn new() -> Self {
            Self {
                hasher: Blake2b256::default(),
            }
        }
        fn reset(&mut self) {
            self.hasher.reset();
        }
        fn init(&mut self) -> Result<(), Self::Error> {
            self.hasher.reset();
            Ok(())
        }
        fn update(&mut self, input: &[u8]) -> Result<(), Self::Error> {
            self.hasher.update(input);
            Ok(())
        }
        fn finalize(&mut self) -> Result<Digest, Self::Error> {
            let digest = self.hasher.clone().finalize();
            Ok(Digest(digest.into()))
        }
    }

    impl<T: Digester> SborEventHandler for TxHashCalculator<T> {
        fn handle(&mut self, evt: SborEvent) {
            self.handle(evt);
        }
    }

    fn calculate_hash_and_compare(input: &[u8], expected_hash: &[u8]) {
        let mut calculator = TxHashCalculator::<TestDigester>::new();
        let mut decoder = SborDecoder::new(true);

        let _ = calculator.start();
        match decoder.decode(&mut calculator, input) {
            Ok(_) => {}
            Err(_) => {
                assert!(false, "Decoder failed");
            }
        }

        let digest = calculator.finalize().unwrap();
        assert_eq!(digest.0, expected_hash);
    }

    #[test]
    fn test_hc_intent() {
        calculate_hash_and_compare(&TX_HC_INTENT, &TX_HC_INTENT_HASH);
    }

    #[test]
    fn test_tx_call_function() {
        calculate_hash_and_compare(&TX_CALL_FUNCTION, &TX_CALL_FUNCTION_HASH);
    }
    #[test]
    fn test_tx_call_method() {
        calculate_hash_and_compare(&TX_CALL_METHOD, &TX_CALL_METHOD_HASH);
    }
    #[test]
    fn test_tx_create_access_controller() {
        calculate_hash_and_compare(
            &TX_CREATE_ACCESS_CONTROLLER,
            &TX_CREATE_ACCESS_CONTROLLER_HASH,
        );
    }
    #[test]
    fn test_tx_create_account() {
        calculate_hash_and_compare(&TX_CREATE_ACCOUNT, &TX_CREATE_ACCOUNT_HASH);
    }
    #[test]
    fn test_tx_create_fungible_resource_with_initial_supply() {
        calculate_hash_and_compare(
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY_HASH,
        );
    }
    #[test]
    fn test_tx_create_fungible_resource_with_no_initial_supply() {
        calculate_hash_and_compare(
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY_HASH,
        );
    }
    #[test]
    fn test_tx_create_identity() {
        calculate_hash_and_compare(&TX_CREATE_IDENTITY, &TX_CREATE_IDENTITY_HASH);
    }
    #[test]
    fn test_tx_create_non_fungible_resource_with_initial_supply() {
        calculate_hash_and_compare(
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY_HASH,
        );
    }
    #[test]
    fn test_tx_create_non_fungible_resource_with_no_initial_supply() {
        calculate_hash_and_compare(
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY_HASH,
        );
    }
    #[test]
    fn test_tx_create_validator() {
        calculate_hash_and_compare(&TX_CREATE_VALIDATOR, &TX_CREATE_VALIDATOR_HASH);
    }
    #[test]
    fn test_tx_metadata() {
        calculate_hash_and_compare(&TX_METADATA, &TX_METADATA_HASH);
    }
    #[test]
    fn test_tx_mint_fungible() {
        calculate_hash_and_compare(&TX_MINT_FUNGIBLE, &TX_MINT_FUNGIBLE_HASH);
    }
    #[test]
    fn test_tx_mint_non_fungible() {
        calculate_hash_and_compare(&TX_MINT_NON_FUNGIBLE, &TX_MINT_NON_FUNGIBLE_HASH);
    }
    #[test]
    fn test_tx_publish_package() {
        calculate_hash_and_compare(&TX_PUBLISH_PACKAGE, &TX_PUBLISH_PACKAGE_HASH);
    }
    #[test]
    fn test_tx_resource_auth_zone() {
        calculate_hash_and_compare(&TX_RESOURCE_AUTH_ZONE, &TX_RESOURCE_AUTH_ZONE_HASH);
    }
    #[test]
    fn test_tx_resource_recall() {
        calculate_hash_and_compare(&TX_RESOURCE_RECALL, &TX_RESOURCE_RECALL_HASH);
    }
    #[test]
    fn test_tx_resource_worktop() {
        calculate_hash_and_compare(&TX_RESOURCE_WORKTOP, &TX_RESOURCE_WORKTOP_HASH);
    }
    #[test]
    fn test_tx_royalty() {
        calculate_hash_and_compare(&TX_ROYALTY, &TX_ROYALTY_HASH);
    }
    #[test]
    fn test_tx_simple_transfer() {
        calculate_hash_and_compare(&TX_SIMPLE_TRANSFER, &TX_SIMPLE_TRANSFER_HASH);
    }
    #[test]
    fn test_tx_simple_transfer_nft() {
        calculate_hash_and_compare(&TX_SIMPLE_TRANSFER_NFT, &TX_SIMPLE_TRANSFER_NFT_HASH);
    }
    #[test]
    fn test_tx_simple_transfer_nft_by_id() {
        calculate_hash_and_compare(
            &TX_SIMPLE_TRANSFER_NFT_BY_ID,
            &TX_SIMPLE_TRANSFER_NFT_BY_ID_HASH,
        );
    }
    #[test]
    fn test_tx_simple_transfer_with_multiple_locked_fees() {
        calculate_hash_and_compare(
            &TX_SIMPLE_TRANSFER_WITH_MULTIPLE_LOCKED_FEES,
            &TX_SIMPLE_TRANSFER_WITH_MULTIPLE_LOCKED_FEES_HASH,
        );
    }
    #[test]
    fn test_tx_access_rule() {
        calculate_hash_and_compare(&TX_ACCESS_RULE, &TX_ACCESS_RULE_HASH);
    }
    #[test]
    fn test_tx_values() {
        calculate_hash_and_compare(&TX_VALUES, &TX_VALUES_HASH);
    }
}
