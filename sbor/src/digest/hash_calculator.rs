use core::result::Result;

use crate::digest::digest::Digest;
use crate::digest::digester::Digester;
use crate::sbor_decoder::SborEvent;

#[repr(u8)]
pub enum HashCalculatorMode {
    Transaction,
    Subintent,
}

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

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
enum SiHashPhase {
    Start,
    Core,
    Header,
    Blobs,
    Message,
    Children,
    ChildrenContent,
    Instructions,
    SingleBlob,
    SingleBlobLen,
    SingleBlobData,
    DecodingError,
    HashingError,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum HashCommitPhase {
    None,
    CommitRegular,
    CommitBlob,
}

pub struct TransactionStateMachine {
    phase: TxHashPhase,
    commit_phase: HashCommitPhase,
}

pub struct SubintentStateMachine {
    phase: SiHashPhase,
    commit_phase: HashCommitPhase,
    input_count: u8,
}

impl TransactionStateMachine {
    pub fn reset(&mut self) {
        self.phase = TxHashPhase::Start;
        self.commit_phase = HashCommitPhase::None;
    }
}

impl SubintentStateMachine {
    pub fn reset(&mut self) {
        self.phase = SiHashPhase::Start;
        self.commit_phase = HashCommitPhase::None;
    }
}

pub struct HashCalculator<T: Digester> {
    work_digester: T,
    blob_digester: T,
    output_digester: T,
    tx_state_machine: TransactionStateMachine,
    si_state_machine: SubintentStateMachine,
    mode: HashCalculatorMode,
}

// Transaction intent hash calculator
impl<T: Digester> HashCalculator<T> {
    fn tx_handle(&mut self, event: SborEvent) {
        match event {
            SborEvent::InputByte(byte) => self.tx_put_byte(byte),
            SborEvent::Start {
                type_id: _,
                nesting_level,
                ..
            } => self.tx_process_start(nesting_level),
            SborEvent::End {
                type_id: _,
                nesting_level,
                ..
            } => self.tx_process_end(nesting_level),
            SborEvent::Len(_) if self.tx_state_machine.phase == TxHashPhase::SingleBlob => {
                self.tx_state_machine.phase = TxHashPhase::SingleBlobLen
            }
            _ => {}
        }
    }

    fn tx_put_byte(&mut self, byte: u8) {
        match self.tx_state_machine.phase {
            TxHashPhase::Start
            | TxHashPhase::DecodingError
            | TxHashPhase::HashingError
            | TxHashPhase::Blobs
            | TxHashPhase::SingleBlob => return,
            TxHashPhase::SingleBlobLen => {
                self.tx_state_machine.phase = TxHashPhase::SingleBlobData;
                return;
            }
            _ => {}
        };

        let digester = if self.tx_state_machine.phase == TxHashPhase::SingleBlobData {
            &mut self.blob_digester
        } else {
            &mut self.work_digester
        };

        match digester.update(&[byte]) {
            Err(..) => self.tx_state_machine.phase = TxHashPhase::HashingError,
            _ => {}
        }

        match self.tx_state_machine.commit_phase {
            HashCommitPhase::None => {}
            HashCommitPhase::CommitRegular => self.tx_finalize_and_push(),
            HashCommitPhase::CommitBlob => {
                self.tx_finalize_and_push_blob();
                self.tx_state_machine.phase = TxHashPhase::Blobs;
            }
        }
        self.tx_state_machine.commit_phase = HashCommitPhase::None;
    }

    fn tx_process_start(&mut self, nesting_level: u8) {
        match (self.tx_state_machine.phase, nesting_level) {
            (TxHashPhase::Start, 1) => self.tx_state_machine.phase = TxHashPhase::Header,
            (TxHashPhase::Header, 1) => self.tx_state_machine.phase = TxHashPhase::Instructions,
            (TxHashPhase::Instructions, 1) => self.tx_state_machine.phase = TxHashPhase::Blobs,
            (TxHashPhase::Blobs, 2) => self.tx_state_machine.phase = TxHashPhase::SingleBlob,
            (TxHashPhase::Blobs, 1) => {
                self.tx_finalize_and_push();
                self.tx_state_machine.commit_phase = HashCommitPhase::None;
                self.tx_state_machine.phase = TxHashPhase::Attachments;
            }
            (TxHashPhase::Attachments, 1) => {
                self.tx_state_machine.phase = TxHashPhase::DecodingError
            }
            (_, _) => {}
        }
    }

    fn tx_process_end(&mut self, nesting_level: u8) {
        match (self.tx_state_machine.phase, nesting_level) {
            (TxHashPhase::Header, 1) => {
                self.tx_state_machine.commit_phase = HashCommitPhase::CommitRegular
            }
            (TxHashPhase::Instructions, 1) => {
                self.tx_state_machine.commit_phase = HashCommitPhase::CommitRegular
            }
            (TxHashPhase::Blobs, 1) => {
                self.tx_state_machine.commit_phase = HashCommitPhase::CommitRegular
            }
            (TxHashPhase::SingleBlobData, 2) => {
                self.tx_state_machine.commit_phase = HashCommitPhase::CommitBlob
            }
            (TxHashPhase::Attachments, 1) => {
                self.tx_state_machine.commit_phase = HashCommitPhase::CommitRegular
            }
            (_, _) => {}
        }
    }

    fn tx_finalize_and_push(&mut self) {
        match self.work_digester.finalize() {
            Ok(digest) => match self.output_digester.update(digest.as_bytes()) {
                Ok(_) => {}
                Err(_) => {
                    self.tx_state_machine.phase = TxHashPhase::HashingError;
                }
            },
            Err(_) => {
                self.tx_state_machine.phase = TxHashPhase::HashingError;
            }
        }

        match self.work_digester.init() {
            Ok(_) => {}
            Err(_) => {
                self.tx_state_machine.phase = TxHashPhase::HashingError;
            }
        }
    }

    fn tx_finalize_and_push_blob(&mut self) {
        match self.blob_digester.finalize() {
            Ok(digest) => match self.work_digester.update(digest.as_bytes()) {
                Ok(_) => {}
                Err(_) => {
                    self.tx_state_machine.phase = TxHashPhase::HashingError;
                }
            },
            Err(_) => {
                self.tx_state_machine.phase = TxHashPhase::HashingError;
            }
        };

        match self.blob_digester.init() {
            Ok(_) => {}
            Err(_) => {
                self.tx_state_machine.phase = TxHashPhase::HashingError;
            }
        }
    }
}

impl<T: Digester> HashCalculator<T> {
    fn si_handle(&mut self, event: SborEvent) {
        #[cfg(test)]
        println!("handle_si: {:?}", event);

        match event {
            SborEvent::InputByte(byte) => self.si_put_byte(byte),
            SborEvent::Start {
                type_id: _,
                nesting_level,
                ..
            } => self.si_process_start(nesting_level),
            SborEvent::End {
                type_id: _,
                nesting_level,
                ..
            } => self.si_process_end(nesting_level),
            SborEvent::Len(_) => {
                if self.si_state_machine.phase == SiHashPhase::SingleBlob {
                    self.si_state_machine.phase = SiHashPhase::SingleBlobLen
                }
                if self.si_state_machine.phase == SiHashPhase::Children {
                    self.si_state_machine.phase = SiHashPhase::ChildrenContent;
                    self.si_state_machine.input_count = 0;
                }
            }
            _ => {}
        }
    }

    fn si_process_start(&mut self, nesting_level: u8) {
        #[cfg(test)]
        println!(
            "si_process_start 1: {:?}, {:?}",
            self.si_state_machine.phase, self.si_state_machine.commit_phase
        );

        let initial_phase = self.si_state_machine.phase;

        match (self.si_state_machine.phase, nesting_level) {
            (SiHashPhase::Start, 0) => self.si_state_machine.phase = SiHashPhase::Core,
            (SiHashPhase::Core, 2) => self.si_state_machine.phase = SiHashPhase::Header,
            (SiHashPhase::Header, 2) => self.si_state_machine.phase = SiHashPhase::Blobs,
            (SiHashPhase::Blobs, 3) => self.si_state_machine.phase = SiHashPhase::SingleBlob,
            (SiHashPhase::Blobs, 2) => {
                self.si_finalize_and_push();
                self.si_state_machine.commit_phase = HashCommitPhase::None;
                self.si_state_machine.phase = SiHashPhase::Message
            }
            (SiHashPhase::Message, 2) => self.si_state_machine.phase = SiHashPhase::Children,
            (SiHashPhase::ChildrenContent, 2) => self.si_state_machine.phase = SiHashPhase::Instructions,
            (SiHashPhase::Instructions, 2) => {
                self.si_state_machine.phase = SiHashPhase::DecodingError
            }
            (_, _) => {}
        }

        if initial_phase != self.si_state_machine.phase {
            self.si_state_machine.input_count = 0;
        }

        #[cfg(test)]
        println!(
            "si_process_start 2: {:?}, {:?}",
            self.si_state_machine.phase, self.si_state_machine.commit_phase
        );
    }

    fn si_process_end(&mut self, nesting_level: u8) {
        #[cfg(test)]
        println!(
            "si_process_end 1: {:?}, {:?}",
            self.si_state_machine.phase, self.si_state_machine.commit_phase
        );
        match (self.si_state_machine.phase, nesting_level) {
            (SiHashPhase::Header, 2) => {
                self.si_state_machine.commit_phase = HashCommitPhase::CommitRegular
            }
            (SiHashPhase::Instructions, 2) => {
                self.si_state_machine.commit_phase = HashCommitPhase::CommitRegular
            }
            (SiHashPhase::Blobs, 2) => {
                self.si_state_machine.commit_phase = HashCommitPhase::CommitRegular
            }
            (SiHashPhase::Message, 2) => {
                self.si_state_machine.commit_phase = HashCommitPhase::CommitRegular
            }
            (SiHashPhase::ChildrenContent, 2) => {
                self.si_state_machine.commit_phase = HashCommitPhase::CommitRegular
            }
            (SiHashPhase::SingleBlobData, 3) => {
                self.si_state_machine.commit_phase = HashCommitPhase::CommitBlob
            }
            (_, _) => {}
        }
        #[cfg(test)]
        println!(
            "si_process_end 2: {:?}, {:?}",
            self.si_state_machine.phase, self.si_state_machine.commit_phase
        );
    }

    fn si_put_byte(&mut self, byte: u8) {
        #[cfg(test)]
        println!(
            "si_put_byte 1: phase {:?}, commit phase {:?}, count {:?}, byte {:?}",
            self.si_state_machine.phase,
            self.si_state_machine.commit_phase,
            self.si_state_machine.input_count,
            byte
        );

        match self.si_state_machine.phase {
            SiHashPhase::Start
            | SiHashPhase::Core
            | SiHashPhase::Children
            | SiHashPhase::DecodingError
            | SiHashPhase::HashingError
            | SiHashPhase::Blobs
            | SiHashPhase::SingleBlob => return,
            SiHashPhase::SingleBlobLen => {
                self.si_state_machine.phase = SiHashPhase::SingleBlobData;
                return;
            }
            _ => {}
        };

        let digester = if self.si_state_machine.phase == SiHashPhase::SingleBlobData {
            &mut self.blob_digester
        } else {
            &mut self.work_digester
        };

        if self.si_state_machine.phase == SiHashPhase::SingleBlobData
            || self.si_state_machine.input_count > 0
        {
            match digester.update(&[byte]) {
                Err(..) => self.si_state_machine.phase = SiHashPhase::HashingError,
                _ => {}
            }
        } else {
            self.si_state_machine.input_count += 1;
        }

        match self.si_state_machine.commit_phase {
            HashCommitPhase::None => {}
            HashCommitPhase::CommitRegular => {
                self.si_finalize_and_push();
                self.si_state_machine.input_count = 0;
            }
            HashCommitPhase::CommitBlob => {
                self.si_finalize_and_push_blob();
                self.si_state_machine.phase = SiHashPhase::Blobs;
            }
        }
        self.si_state_machine.commit_phase = HashCommitPhase::None;
    }

    fn si_finalize_and_push(&mut self) {
        match self.work_digester.finalize() {
            Ok(digest) => match self.output_digester.update(digest.as_bytes()) {
                Ok(_) => {
                    #[cfg(test)]
                    {
                        let blob = digest.as_bytes();
                        print!("{:?} = ", self.si_state_machine.phase);

                        for &byte in blob.iter() {
                            print!("{:#04x}, ", byte);
                        }

                        println!();
                    }
                }
                Err(_) => {
                    self.si_state_machine.phase = SiHashPhase::HashingError;
                }
            },
            Err(_) => {
                self.si_state_machine.phase = SiHashPhase::HashingError;
            }
        }

        match self.work_digester.init() {
            Ok(_) => {}
            Err(_) => {
                self.si_state_machine.phase = SiHashPhase::HashingError;
            }
        }
    }

    fn si_finalize_and_push_blob(&mut self) {
        match self.blob_digester.finalize() {
            Ok(digest) => match self.work_digester.update(digest.as_bytes()) {
                Ok(_) => {}
                Err(_) => {
                    self.si_state_machine.phase = SiHashPhase::HashingError;
                }
            },
            Err(_) => {
                self.si_state_machine.phase = SiHashPhase::HashingError;
            }
        };

        match self.blob_digester.init() {
            Ok(_) => {}
            Err(_) => {
                self.si_state_machine.phase = SiHashPhase::HashingError;
            }
        }
    }
}

// Common part + externally visible API for both transaction and subintent hash calculators
impl<T: Digester> HashCalculator<T> {
    const PAYLOAD_PREFIX: u8 = 0x54;
    const V1_INTENT: u8 = 1;
    const V2_SUBINTENT: u8 = 11;
    const TX_INITIAL_VECTOR: [u8; 2] = [Self::PAYLOAD_PREFIX, Self::V1_INTENT];
    const SI_INITIAL_VECTOR: [u8; 2] = [Self::PAYLOAD_PREFIX, Self::V2_SUBINTENT];

    pub fn new() -> Self {
        Self {
            work_digester: T::new(),
            blob_digester: T::new(),
            output_digester: T::new(),
            tx_state_machine: TransactionStateMachine {
                phase: TxHashPhase::Start,
                commit_phase: HashCommitPhase::None,
            },
            si_state_machine: SubintentStateMachine {
                phase: SiHashPhase::Start,
                commit_phase: HashCommitPhase::None,
                input_count: 0,
            },
            mode: HashCalculatorMode::Transaction,
        }
    }

    // Reuse digester for auth digest calculation
    #[inline(always)]
    pub fn auth_digest(
        &mut self,
        challenge: &[u8],
        address: &[u8],
        origin: &[u8],
    ) -> Result<Digest, T::Error> {
        self.reset();
        self.work_digester.init()?;
        self.work_digester.update(&[82u8])?;
        self.work_digester.update(challenge)?;
        self.work_digester.update(&[address.len() as u8])?;
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
        self.tx_state_machine.reset();
        self.si_state_machine.reset();
        self.mode = HashCalculatorMode::Transaction;
    }

    pub fn start(&mut self, mode: HashCalculatorMode) -> Result<(), T::Error> {
        self.mode = mode;

        self.work_digester.init()?;
        self.blob_digester.init()?;
        self.output_digester.init()?;

        let init_vector = match self.mode {
            HashCalculatorMode::Transaction => &Self::TX_INITIAL_VECTOR,
            HashCalculatorMode::Subintent => &Self::SI_INITIAL_VECTOR,
        };

        self.output_digester.update(init_vector)
    }

    pub fn handle(&mut self, event: SborEvent) {
        match self.mode {
            HashCalculatorMode::Transaction => self.tx_handle(event),
            HashCalculatorMode::Subintent => self.si_handle(event),
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
    use crate::digest::hash_calculator::{HashCalculator, HashCalculatorMode};
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

    impl<T: Digester> SborEventHandler for HashCalculator<T> {
        fn handle(&mut self, evt: SborEvent) {
            self.handle(evt);
        }
    }

    fn calculate_tx_hash_and_compare(input: &[u8], expected_hash: &[u8]) {
        let mut calculator = HashCalculator::<TestDigester>::new();
        let mut decoder = SborDecoder::new(true);

        let _ = calculator.start(HashCalculatorMode::Transaction);
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
        calculate_tx_hash_and_compare(&TX_HC_INTENT, &TX_HC_INTENT_HASH);
    }

    #[test]
    fn test_tx_call_function() {
        calculate_tx_hash_and_compare(&TX_CALL_FUNCTION, &TX_CALL_FUNCTION_HASH);
    }
    #[test]
    fn test_tx_call_method() {
        calculate_tx_hash_and_compare(&TX_CALL_METHOD, &TX_CALL_METHOD_HASH);
    }
    #[test]
    fn test_tx_create_access_controller() {
        calculate_tx_hash_and_compare(
            &TX_CREATE_ACCESS_CONTROLLER,
            &TX_CREATE_ACCESS_CONTROLLER_HASH,
        );
    }
    #[test]
    fn test_tx_create_account() {
        calculate_tx_hash_and_compare(&TX_CREATE_ACCOUNT, &TX_CREATE_ACCOUNT_HASH);
    }
    #[test]
    fn test_tx_create_fungible_resource_with_initial_supply() {
        calculate_tx_hash_and_compare(
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY_HASH,
        );
    }
    #[test]
    fn test_tx_create_fungible_resource_with_no_initial_supply() {
        calculate_tx_hash_and_compare(
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
            &TX_CREATE_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY_HASH,
        );
    }
    #[test]
    fn test_tx_create_identity() {
        calculate_tx_hash_and_compare(&TX_CREATE_IDENTITY, &TX_CREATE_IDENTITY_HASH);
    }
    #[test]
    fn test_tx_create_non_fungible_resource_with_initial_supply() {
        calculate_tx_hash_and_compare(
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY,
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_INITIAL_SUPPLY_HASH,
        );
    }
    #[test]
    fn test_tx_create_non_fungible_resource_with_no_initial_supply() {
        calculate_tx_hash_and_compare(
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY,
            &TX_CREATE_NON_FUNGIBLE_RESOURCE_WITH_NO_INITIAL_SUPPLY_HASH,
        );
    }
    #[test]
    fn test_tx_create_validator() {
        calculate_tx_hash_and_compare(&TX_CREATE_VALIDATOR, &TX_CREATE_VALIDATOR_HASH);
    }
    #[test]
    fn test_tx_metadata() {
        calculate_tx_hash_and_compare(&TX_METADATA, &TX_METADATA_HASH);
    }
    #[test]
    fn test_tx_mint_fungible() {
        calculate_tx_hash_and_compare(&TX_MINT_FUNGIBLE, &TX_MINT_FUNGIBLE_HASH);
    }
    #[test]
    fn test_tx_mint_non_fungible() {
        calculate_tx_hash_and_compare(&TX_MINT_NON_FUNGIBLE, &TX_MINT_NON_FUNGIBLE_HASH);
    }
    #[test]
    fn test_tx_publish_package() {
        calculate_tx_hash_and_compare(&TX_PUBLISH_PACKAGE, &TX_PUBLISH_PACKAGE_HASH);
    }
    #[test]
    fn test_tx_resource_auth_zone() {
        calculate_tx_hash_and_compare(&TX_RESOURCE_AUTH_ZONE, &TX_RESOURCE_AUTH_ZONE_HASH);
    }
    #[test]
    fn test_tx_resource_recall() {
        calculate_tx_hash_and_compare(&TX_RESOURCE_RECALL, &TX_RESOURCE_RECALL_HASH);
    }
    #[test]
    fn test_tx_resource_recall_nonfungibles() {
        calculate_tx_hash_and_compare(
            &TX_RESOURCE_RECALL_NONFUNGIBLES,
            &TX_RESOURCE_RECALL_NONFUNGIBLES_HASH,
        );
    }
    #[test]
    fn test_tx_resource_worktop() {
        calculate_tx_hash_and_compare(&TX_RESOURCE_WORKTOP, &TX_RESOURCE_WORKTOP_HASH);
    }
    #[test]
    fn test_tx_royalty() {
        calculate_tx_hash_and_compare(&TX_ROYALTY, &TX_ROYALTY_HASH);
    }
    #[test]
    fn test_tx_simple_transfer() {
        calculate_tx_hash_and_compare(&TX_SIMPLE_TRANSFER, &TX_SIMPLE_TRANSFER_HASH);
    }
    #[test]
    fn test_tx_simple_invalid_transfer() {
        calculate_tx_hash_and_compare(
            &TX_SIMPLE_INVALID_TRANSFER,
            &TX_SIMPLE_INVALID_TRANSFER_HASH,
        );
    }
    #[test]
    fn test_tx_simple_transfer_new_format() {
        calculate_tx_hash_and_compare(
            &TX_SIMPLE_TRANSFER_NEW_FORMAT,
            &TX_SIMPLE_TRANSFER_NEW_FORMAT_HASH,
        );
    }
    #[test]
    fn test_tx_simple_transfer_nft() {
        calculate_tx_hash_and_compare(&TX_SIMPLE_TRANSFER_NFT, &TX_SIMPLE_TRANSFER_NFT_HASH);
    }
    #[test]
    fn test_tx_simple_transfer_nft_new_format() {
        calculate_tx_hash_and_compare(
            &TX_SIMPLE_TRANSFER_NFT_NEW_FORMAT,
            &TX_SIMPLE_TRANSFER_NFT_NEW_FORMAT_HASH,
        );
    }
    #[test]
    fn test_tx_simple_transfer_nft_by_id() {
        calculate_tx_hash_and_compare(
            &TX_SIMPLE_TRANSFER_NFT_BY_ID,
            &TX_SIMPLE_TRANSFER_NFT_BY_ID_HASH,
        );
    }
    #[test]
    fn test_tx_simple_transfer_nft_by_id_new_format() {
        calculate_tx_hash_and_compare(
            &TX_SIMPLE_TRANSFER_NFT_BY_ID_NEW_FORMAT,
            &TX_SIMPLE_TRANSFER_NFT_BY_ID_NEW_FORMAT_HASH,
        );
    }
    #[test]
    fn test_tx_simple_transfer_with_multiple_locked_fees() {
        calculate_tx_hash_and_compare(
            &TX_SIMPLE_TRANSFER_WITH_MULTIPLE_LOCKED_FEES,
            &TX_SIMPLE_TRANSFER_WITH_MULTIPLE_LOCKED_FEES_HASH,
        );
    }
    #[test]
    fn test_tx_access_rule() {
        calculate_tx_hash_and_compare(&TX_ACCESS_RULE, &TX_ACCESS_RULE_HASH);
    }
    #[test]
    fn test_tx_values() {
        calculate_tx_hash_and_compare(&TX_VALUES, &TX_VALUES_HASH);
    }

    //-----------------------------------------------------------------------------------
    // Auth
    //-----------------------------------------------------------------------------------

    struct AuthData {
        blake_hash_of_payload: &'static str,
        dapp_definition_address: &'static str,
        origin: &'static str,
        challenge: &'static str,
    }

    const AUTH_TEST_VECTOR: &[AuthData] = &[
        AuthData {
            blake_hash_of_payload:
                "dc47fc69e9e45855addf579f398da0309c878092dd95352b9fe187a7e5a529e2",
            dapp_definition_address:
                "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
            origin: "https://dashboard.rdx.works",
            challenge: "ec5dcb3d1f75627be1021cb8890f0e8ce0c9fe7f2ff55cbdff096b38a32612c9",
        },
        AuthData {
            blake_hash_of_payload:
                "866836f5b9c827ca38fd2bfef94f95ba21933f75a0291c85d3ecfc18b8aa5b2d",
            dapp_definition_address:
                "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
            origin: "https://dashboard.rdx.works",
            challenge: "d7fb740b9ff00657d710dcbeddb2d432e697fc0dd39c60feb7858b17ef0eff58",
        },
        AuthData {
            blake_hash_of_payload:
                "0f41aa92e8c978d7f920ca56daf123a0a0d975eea06ecfb57bec0a0560fb73e3",
            dapp_definition_address:
                "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
            origin: "https://dashboard.rdx.works",
            challenge: "4aaa2ec25c3fe215412b3f005e4c37d518af3a22b4728587cf6dbcf83341e8b3",
        },
        AuthData {
            blake_hash_of_payload:
                "9c8d2622cedb9dc4e53daea398dd178a2ec938d402eeaba41a2ac946b0f4dd57",
            dapp_definition_address:
                "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
            origin: "https://stella.swap",
            challenge: "a10fad201666b4bcf7f707841d58b11740c290e03790b17ed0fec23b3f180e65",
        },
        AuthData {
            blake_hash_of_payload:
                "2c07a4fc72341ae9160a8f9ddf2d0bb8fd9d795ed0d87059a9e5de8321513871",
            dapp_definition_address:
                "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
            origin: "https://stella.swap",
            challenge: "718b0eb060a719492011910258a4b4119d8c95aef34eb9519c9fa7de25f7ac43",
        },
        AuthData {
            blake_hash_of_payload:
                "306b2407e8b675bb22b630efa938249595433975276862e9bfa07f7f94ca84a8",
            dapp_definition_address:
                "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
            origin: "https://stella.swap",
            challenge: "9a4f834aefdc455cb4601337227e1b7e74d60308327564ececf33456509964cd",
        },
        AuthData {
            blake_hash_of_payload:
                "a14942b1dc361c7e153e4d4200f902da1dafa2bd54bc4c0387c779c22a1e454e",
            dapp_definition_address:
                "account_tdx_b_1p9dkged3rpzy860ampt5jpmvv3yl4y6f5yppp4tnscdslvt9v3",
            origin: "https://rola.xrd",
            challenge: "00dca15875839ab1f549445a36c7b5c0dcf7aebfa7d48f945f2aa5cf4aa1a9a3",
        },
        AuthData {
            blake_hash_of_payload:
                "6a13329619caafdf4351d1c8b85b7f523ce2955873f003402be6e1e45cdce4ae",
            dapp_definition_address:
                "account_tdx_b_1p8ahenyznrqy2w0tyg00r82rwuxys6z8kmrhh37c7maqpydx7p",
            origin: "https://rola.xrd",
            challenge: "0a510b2362c9ce19d11c538b2f6a15f62caab6528071eaad5ba8a563a02e01cb",
        },
        AuthData {
            blake_hash_of_payload:
                "f9ec8f328d9aeec55546d1cd78a13cc7967bd52aba3c8e305ed39f82465f395c",
            dapp_definition_address:
                "account_tdx_b_1p95nal0nmrqyl5r4phcspg8ahwnamaduzdd3kaklw3vqeavrwa",
            origin: "https://rola.xrd",
            challenge: "20619c1df905a28e7a76d431f2b59e99dd1a8f386842e1701862e765806a5c47",
        },
    ];

    use core::fmt::Write;
    use core::num::ParseIntError;

    fn calculate_auth_and_compare(input: &AuthData) {
        fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
            (0..s.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
                .collect()
        }

        pub fn encode_hex(bytes: &[u8]) -> String {
            let mut s = String::with_capacity(bytes.len() * 2);
            for &b in bytes {
                write!(&mut s, "{:02x}", b).unwrap();
            }
            s
        }

        let expected_hash = decode_hex(input.blake_hash_of_payload).unwrap();
        let challenge = decode_hex(input.challenge).unwrap();

        let digest = HashCalculator::<TestDigester>::new()
            .auth_digest(
                challenge.as_slice(),
                input.dapp_definition_address.as_bytes(),
                input.origin.as_bytes(),
            )
            .unwrap();

        println!("expected: {}", encode_hex(expected_hash.as_slice()));
        println!("actual  : {}", encode_hex(&digest.0));
        assert_eq!(digest.0, expected_hash.as_slice());
    }

    #[test]
    fn test_auth_digest() {
        for input in AUTH_TEST_VECTOR.iter() {
            calculate_auth_and_compare(input);
        }
    }

    //-----------------------------------------------------------------------------------
    // Subintent
    //-----------------------------------------------------------------------------------

    use crate::si_test_data::tests::*;

    fn calculate_si_hash_and_compare(input: &[u8], expected_hash: &[u8]) {
        let mut calculator = HashCalculator::<TestDigester>::new();
        let mut decoder = SborDecoder::new(true);

        let _ = calculator.start(HashCalculatorMode::Subintent);
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
    fn test_si_checked_childless_subintent() {
        calculate_si_hash_and_compare(
            &SI_CHECKED_CHILDLESS_SUBINTENT,
            &SI_CHECKED_CHILDLESS_SUBINTENT_HASH,
        );
    }
}
