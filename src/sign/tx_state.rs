use nanos_sdk::io::Comm;
use sbor::decoder_error::DecoderError;
use sbor::math::Decimal;
use sbor::print::instruction_printer::DetectedTxType;
use sbor::print::tty::TTY;
use sbor::sbor_decoder::{DecodingOutcome, SborDecoder};

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::crypto::hash::Digest;
use crate::sign::instruction_processor::InstructionProcessor;
use crate::sign::sign_outcome::SignOutcome;
use crate::sign::sign_type::SignType;
use crate::ui::multiline_scroller::MultilineMessageScroller;
use crate::ui::multipage_validator::MultipageValidator;
use crate::ui::single_message::SingleMessage;
use crate::utilities::conversion::{lower_as_hex, upper_as_hex};

const NONCE_LENGTH: usize = 32;
const DAPP_ADDRESS_LENGTH: usize = 70;
const ORIGIN_LENGTH: usize = 150;
const MIN_DAPP_ADDRESS_LENGTH: usize = 2; // 1 byte length + 1 byte address
const MIN_ORIGIN_LENGTH: usize = 10; // 1 byte length + "https://a"
const MIN_VALID_LENGTH: usize = NONCE_LENGTH + MIN_DAPP_ADDRESS_LENGTH + MIN_ORIGIN_LENGTH;

pub struct TxState<T> {
    decoder: SborDecoder,
    processor: InstructionProcessor<T>,
    show_digest: bool,
}

impl<T> TxState<T> {
    pub fn new(tty: TTY<T>) -> Self {
        Self {
            decoder: SborDecoder::new(true),
            processor: InstructionProcessor::new(tty),
            show_digest: false,
        }
    }

    pub fn reset(&mut self) {
        self.processor.reset();
        self.decoder.reset();
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
        if class == CommandClass::Regular {
            self.reset();
            self.processor.process_sign(comm, class, tx_type)?;
            self.processor.set_network()?;
            self.processor.set_show_instructions();
            self.show_digest = match tx_type {
                SignType::Ed25519 | SignType::Secp256k1 => comm.get_apdu_metadata().p1 == 1,
                SignType::Ed25519Summary | SignType::Secp256k1Summary => false,
                SignType::AuthEd25519 | SignType::AuthSecp256k1 => false,
            };
            self.show_introductory_screen(tx_type)?;
        } else {
            self.processor.process_sign(comm, class, tx_type)?;

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

    fn process_sign_auth(
        &mut self,
        value: &[u8],
        tx_type: SignType,
    ) -> Result<SignOutcome, AppError> {
        if value.len() < MIN_VALID_LENGTH {
            return Err(AppError::BadAuthSignRequest);
        }

        let nonce = &value[..NONCE_LENGTH];
        let addr_start = NONCE_LENGTH + 1;
        let addr_end = addr_start + value[NONCE_LENGTH] as usize;
        let address = &value[addr_start..addr_end];
        let hash_address = &value[NONCE_LENGTH..addr_end];
        let origin = &value[addr_end..];

        let mut nonce_hex = [0u8; NONCE_LENGTH * 2];

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
            true,
        )
        .event_loop();
    }

    fn fee_info_message(&mut self, fee: &Decimal) {
        let text = self.processor.format_decimal(fee);

        MultilineMessageScroller::with_title(
            "Max TX Fee:",
            core::str::from_utf8(text).unwrap(),
            true,
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
        self.display_tx_info(tx_type, &digest);

        let rc = MultipageValidator::new(&[&"Sign TX?"], &[&"Sign"], &[&"Reject"]).ask();

        if rc {
            self.processor.sign_tx(tx_type, digest)
        } else {
            return Ok(SignOutcome::SigningRejected);
        }
    }

    fn show_transaction_fee(&mut self, detected_type: &DetectedTxType) {
        match detected_type {
            DetectedTxType::OtherWithFee(fee) | DetectedTxType::TransferWithFee(fee) => {
                self.fee_info_message(fee);
            }
            _ => {
                // Ignore remaining types as detection was not requested
            }
        }
    }

    fn show_detected_tx_type(&mut self, detected_type: &DetectedTxType) {
        let text: &[u8] = match detected_type {
            DetectedTxType::Other | DetectedTxType::OtherWithFee(..) => b"Other",
            DetectedTxType::Transfer | DetectedTxType::TransferWithFee(..) => b"Transfer",
        };
        self.info_message(b"TX Type:", text);
    }

    fn show_digest(&mut self, digest: &Digest) {
        if self.show_digest {
            self.info_message(b"TX Hash:", &digest.as_hex());
        }
    }

    fn display_tx_info(&mut self, tx_type: SignType, digest: &Digest) {
        let detected_type = self.processor.get_detected_tx_type();

        match tx_type {
            SignType::Ed25519 | SignType::Secp256k1 => {
                self.show_transaction_fee(&detected_type);
                self.show_digest(digest);
            }
            SignType::Ed25519Summary | SignType::Secp256k1Summary => {
                self.show_detected_tx_type(&detected_type);
                self.show_transaction_fee(&detected_type);

                self.show_digest = match detected_type {
                    DetectedTxType::Transfer | DetectedTxType::TransferWithFee(..) => false,
                    DetectedTxType::Other | DetectedTxType::OtherWithFee(..) => true,
                };

                self.show_digest(digest);
            }
            SignType::AuthEd25519 | SignType::AuthSecp256k1 => {}
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
