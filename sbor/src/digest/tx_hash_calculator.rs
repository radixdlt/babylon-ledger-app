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

    const HC_INTENT_HASH: [u8; 32] = [
        0xd5, 0xf6, 0x7c, 0x03, 0xd5, 0x69, 0x39, 0x01, 0xa0, 0x87, 0x32, 0x35, 0x94, 0x52, 0x10,
        0x7b, 0x63, 0x85, 0xe4, 0xb7, 0x6c, 0x9e, 0xdb, 0xf1, 0xab, 0x10, 0x74, 0xc6, 0x40, 0x57,
        0x8b, 0x24,
    ];
    const HC_INTENT: [u8; 95] = [
        0x4d, 0x22, 0x01, 0x04, 0x21, 0x07, 0x07, 0xf2, 0x0a, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x0a, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00,
        0x00, 0x22, 0x01, 0x01, 0x20, 0x07, 0x20, 0xf3, 0x81, 0x62, 0x6e, 0x41, 0xe7, 0x02, 0x7e,
        0xa4, 0x31, 0xbf, 0xe3, 0x00, 0x9e, 0x94, 0xbd, 0xd2, 0x5a, 0x74, 0x6b, 0xee, 0xc4, 0x68,
        0x94, 0x8d, 0x6c, 0x3c, 0x7c, 0x5d, 0xc9, 0xa5, 0x4b, 0x01, 0x00, 0x08, 0x00, 0x00, 0x20,
        0x22, 0x01, 0x12, 0x00, 0x20, 0x20, 0x02, 0x07, 0x04, 0x00, 0x01, 0x02, 0x03, 0x07, 0x02,
        0x05, 0x06, 0x22, 0x00, 0x00,
    ];

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

    #[test]
    fn test_v1_user_transaction_structure() {
        let mut decoder = SborDecoder::new(true);
        let mut calculator = TxHashCalculator::<TestDigester>::new();

        calculator.start();

        decoder.decode(&mut calculator, &HC_INTENT);

        let digest = calculator.finalize().unwrap();
        assert_eq!(digest.0, HC_INTENT_HASH);
    }
}
