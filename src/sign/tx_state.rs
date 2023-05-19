use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::hash::Digest;
use crate::ledger_display_io::LedgerTTY;
use crate::sign::instruction_processor::InstructionProcessor;
use crate::sign::sign_outcome::SignOutcome;
use crate::sign::sign_type::SignType;
use crate::ui::multiline_scroller::MultilineMessageScroller;
use crate::ui::multipage_validator::MultipageValidator;
use crate::ui::single_message::SingleMessage;
use crate::utilities::conversion::{lower_as_hex, upper_as_hex};
use nanos_sdk::io::Comm;
use sbor::decoder_error::DecoderError;
use sbor::sbor_decoder::{DecodingOutcome, SborDecoder};

pub struct TxState {
    decoder: SborDecoder,
    processor: InstructionProcessor,
    show_digest: bool,
}

impl TxState {
    pub fn new() -> Self {
        Self {
            decoder: SborDecoder::new(true),
            processor: InstructionProcessor::new(),
            show_digest: false,
        }
    }

    pub fn reset(&mut self) {
        self.processor.reset();
        self.decoder.reset();
        self.processor.set_tty(LedgerTTY::new_tty());
    }

    pub fn process_sign(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignType,
    ) -> Result<SignOutcome, AppError> {
        let result = self.process_sign_internal(comm, class, tx_type);

        match result {
            Ok(outcome) => match outcome {
                SignOutcome::SendNextPacket => result,
                _ => {
                    self.reset(); // Ensure state is reset on error or end of processing
                    result
                }
            },
            Err(_) => {
                self.reset(); // Ensure state is reset on error
                result
            }
        }
    }

    fn process_sign_internal(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        tx_type: SignType,
    ) -> Result<SignOutcome, AppError> {
        self.processor.process_sign(comm, class, tx_type)?;

        if class == CommandClass::Regular {
            self.processor.set_network()?;
            self.processor.set_show_instructions();
            self.show_digest = match tx_type {
                SignType::Ed25519 | SignType::Secp256k1 => comm.get_apdu_metadata().p1 == 1,
                SignType::Ed25519Summary | SignType::Secp256k1Summary => true,
                SignType::AuthEd25519 | SignType::AuthSecp256k1 => false,
            };
            self.show_introductory_screen(tx_type)?;
        } else {
            match tx_type {
                SignType::AuthEd25519 | SignType::AuthSecp256k1 => {
                    return if class != CommandClass::LastData {
                        Err(AppError::BadAuthSignSequence)
                    } else {
                        self.process_sign_auth(comm.get_data()?, tx_type)
                    }
                }
                _ => self.decode_tx_intent(comm.get_data()?, class)?,
            }
        }

        if class == CommandClass::LastData {
            self.finalize_sign_tx(tx_type)
        } else {
            Ok(SignOutcome::SendNextPacket)
        }
    }

    fn show_introductory_screen(&mut self, tx_type: SignType) -> Result<(), AppError> {
        let text = match tx_type {
            SignType::Ed25519
            | SignType::Secp256k1
            | SignType::Ed25519Summary
            | SignType::Secp256k1Summary => "Review\n\nTransaction",
            SignType::AuthEd25519 | SignType::AuthSecp256k1 => "Review\nOwnership\nProof",
        };

        SingleMessage::with_right_arrow(text).show_and_wait();

        Ok(())
    }

    const NONCE_LENGTH: usize = 32;
    const DAPP_ADDRESS_LENGTH: usize = 70;
    const ORIGIN_LENGTH: usize = 150;
    const MIN_DAPP_ADDRESS_LENGTH: usize = 2; // 1 byte length + 1 byte address
    const MIN_ORIGIN_LENGTH: usize = 10; // 1 byte length + "https://a"
    const MIN_VALID_LENGTH: usize =
        Self::NONCE_LENGTH + Self::MIN_DAPP_ADDRESS_LENGTH + Self::MIN_ORIGIN_LENGTH;

    fn process_sign_auth(
        &mut self,
        value: &[u8],
        tx_type: SignType,
    ) -> Result<SignOutcome, AppError> {
        if value.len() < Self::MIN_VALID_LENGTH {
            return Err(AppError::BadAuthSignRequest);
        }

        let nonce = &value[..Self::NONCE_LENGTH];
        let addr_start = Self::NONCE_LENGTH + 1;
        let addr_end = addr_start + value[Self::NONCE_LENGTH] as usize;
        let address = &value[addr_start..addr_end];
        let hash_address = &value[Self::NONCE_LENGTH..addr_end];
        let origin = &value[addr_end..];

        let mut nonce_hex = [0u8; Self::NONCE_LENGTH * 2];

        for (i, &byte) in nonce.iter().enumerate() {
            nonce_hex[i * 2] = upper_as_hex(byte);
            nonce_hex[i * 2 + 1] = lower_as_hex(byte);
        }

        self.info_message(b"Origin:", origin);
        self.info_message(b"dApp Address:", address);
        self.info_message(b"Nonce:", &nonce_hex);

        let rc = MultipageValidator::new(&[&"Sign Proof?"], &[&"Sign"], &[&"Reject"]).ask();

        if rc {
            let digest = self.auth_digest(nonce, hash_address, origin)?;
            self.processor.sign_tx(tx_type, digest)
        } else {
            return Ok(SignOutcome::SigningRejected);
        }
    }

    fn info_message(&mut self, title: &[u8], message: &[u8]) {
        MultilineMessageScroller::with_title(
            core::str::from_utf8(title).unwrap(),
            core::str::from_utf8(message).unwrap(),
            true
        )
        .event_loop();
    }

    fn auth_digest(
        &mut self,
        nonce: &[u8],
        address: &[u8],
        origin: &[u8],
    ) -> Result<Digest, AppError> {
        self.processor.auth_digest(nonce, address, origin)
    }

    fn finalize_sign_tx(&mut self, tx_type: SignType) -> Result<SignOutcome, AppError> {
        let digest = self.processor.finalize()?;

        if self.show_digest {
            self.info_message(b"Digest:", &digest.as_hex());
        }

        let rc = MultipageValidator::new(&[&"Sign TX?"], &[&"Sign"], &[&"Reject"]).ask();

        if rc {
            self.processor.sign_tx(tx_type, digest)
        } else {
            return Ok(SignOutcome::SigningRejected);
        }
    }

    fn decode_tx_intent(&mut self, data: &[u8], class: CommandClass) -> Result<(), AppError> {
        let result = self.call_decoder(data);

        match result {
            Ok(outcome) => self.validate_outcome(class, outcome),
            Err(err) => Err(err.into()),
        }
    }

    fn validate_outcome(
        &mut self,
        class: CommandClass,
        outcome: DecodingOutcome,
    ) -> Result<(), AppError> {
        match (outcome, class) {
            // Decoding done and it was last data packet
            (DecodingOutcome::Done(size), CommandClass::LastData)
                if size == self.processor.tx_size() =>
            {
                Ok(())
            }
            // Decoding is incomplete and it was first packet or continuation packet
            (DecodingOutcome::NeedMoreData(size), CommandClass::Regular)
            | (DecodingOutcome::NeedMoreData(size), CommandClass::Continuation)
                if size == self.processor.tx_size() =>
            {
                Ok(())
            }
            // All other combinations are invalid
            _ => Err(AppError::BadTxSignLen),
        }
    }

    fn call_decoder(&mut self, data: &[u8]) -> Result<DecodingOutcome, DecoderError> {
        self.decoder.decode(&mut self.processor, data)
    }
}
