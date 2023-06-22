use core::result::Result;

use crate::digest::digest::Digest;
use crate::digest::digester::Digester;
use crate::sbor_decoder::SborEvent;

#[derive(Copy, Clone)]
#[repr(u8)]
enum TxHashPhase {
    Start,
    Header,
    Instructions,
    Blobs,
    SingleBlob,
    Attachments,
    DecodingError,
    HashingError,
}

pub struct TxHashCalculator<T: Digester> {
    work_digester: T,
    blob_digester: T,
    output_digester: T,
    phase: TxHashPhase,
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
                type_id,
                nesting_level,
                ..
            } => self.process_start(type_id, nesting_level),
            SborEvent::End {
                type_id,
                nesting_level,
                ..
            } => self.process_end(type_id, nesting_level),
            _ => {}
        }
    }

    fn put_byte(&mut self, byte: u8) {
        match self.work_digester.update(&[byte]) {
            Err(..) => self.phase = TxHashPhase::HashingError,
            _ => {}
        }
    }

    fn process_start(&mut self, type_id: u8, nesting_level: u8) {
        match (self.phase, nesting_level) {
            (TxHashPhase::Start, 1) => self.phase = TxHashPhase::Header,
            (TxHashPhase::Header, 1) => self.phase = TxHashPhase::Instructions,
            (TxHashPhase::Instructions, 1) => self.phase = TxHashPhase::Blobs,
            (TxHashPhase::Blobs, 2) => self.phase = TxHashPhase::SingleBlob,
            (TxHashPhase::Blobs, 1) => self.phase = TxHashPhase::Attachments,
            (TxHashPhase::Attachments, 1) => self.phase = TxHashPhase::DecodingError,
            (_, _) => {}
        }
    }

    fn process_end(&mut self, type_id: u8, nesting_level: u8) {
        match (self.phase, nesting_level) {
            (TxHashPhase::Header, 1) => self.finalize_and_push(),
            (TxHashPhase::Instructions, 1) => self.finalize_and_push(),
            (TxHashPhase::Blobs, 1) => self.finalize_and_push(),
            (TxHashPhase::SingleBlob, 2) => self.finalize_and_push_blob(),
            (TxHashPhase::Attachments, 1) => self.finalize_and_push(),
            (_, _) => {}
        }
    }

    fn finalize_and_push(&mut self) {
        match self.work_digester.finalize() {
            Ok(digest) => {
                self.output_digester.update(digest.as_bytes());
            }
            Err(_) => {
                self.phase = TxHashPhase::HashingError;
            }
        }

        let _ = self.work_digester.init();
    }

    fn finalize_and_push_blob(&mut self) {
        match self.blob_digester.finalize() {
            Ok(digest) => {
                self.work_digester.update(digest.as_bytes());
            }
            Err(_) => {
                self.phase = TxHashPhase::HashingError;
            }
        };

        let _ = self.blob_digester.init();
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

    pub type Blake2b256 = Blake2b<U32>;

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

        decoder.decode(&mut calculator, &TX_CALL_FUNCTION);
    }
}
