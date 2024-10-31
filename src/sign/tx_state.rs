use crate::io::Comm;
use sbor::decoder_error::DecoderError;
use sbor::digest::digest::Digest;
use sbor::print::tty::TTY;
use sbor::print::tx_intent_type::TxIntentType;
use sbor::print::tx_summary_detector::DetectedTxType;
use sbor::sbor_decoder::{DecodingOutcome, SborDecoder};
use sbor::utilities::conversion::{lower_as_hex, upper_as_hex};

use crate::app_error::AppError;
use crate::command_class::CommandClass;
use crate::settings::Settings;
use crate::sign::decoding_mode::DecodingMode;
use crate::sign::instruction_processor::InstructionProcessor;
use crate::sign::sign_mode::SignMode;
use crate::sign::sign_outcome::SignOutcome;
use crate::xui::{
    auth_details, fee, hash, introductory_screen, pre_auth_hash_details, signature, transfer,
};

const AUTH_CHALLENGE_LENGTH: usize = 32;
const AUTH_MIN_DAPP_ADDRESS_LENGTH: usize = 2; // 1 byte length + 1 byte address
const AUTH_MIN_ORIGIN_LENGTH: usize = 10; // 1 byte length + "https://a"
const AUTH_MIN_VALID_LENGTH: usize =
    AUTH_CHALLENGE_LENGTH + AUTH_MIN_DAPP_ADDRESS_LENGTH + AUTH_MIN_ORIGIN_LENGTH;
const SUBINTENT_MESSAGE_LENGTH: usize = Digest::DIGEST_LENGTH;

pub struct TxState<T: Copy> {
    decoder: SborDecoder,
    processor: InstructionProcessor<T>,
}

impl<T: Copy> TxState<T> {
    pub fn new(tty: TTY<T>) -> Self {
        Self {
            decoder: SborDecoder::new(true),
            processor: InstructionProcessor::new(tty),
        }
    }

    pub fn reset(&mut self) {
        self.processor.reset();
        self.decoder.reset();
    }

    pub fn send_settings(&self, comm: &mut Comm) -> Result<(), AppError> {
        comm.append(&Settings::get().as_bytes());

        Ok(())
    }

    pub fn process_sign_with_mode(
        &mut self,
        comm: &mut Comm,
        class: CommandClass,
        sign_mode: SignMode,
        intent_type: TxIntentType,
    ) -> Result<SignOutcome, AppError> {
        let result = self.process_sign_internal(comm, class, sign_mode, intent_type);

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
        sign_mode: SignMode,
        intent_type: TxIntentType,
    ) -> Result<SignOutcome, AppError> {
        if class == CommandClass::Regular {
            // Start of the processing
            self.reset();
            self.processor.set_intent_type(intent_type);
            self.processor.process_sign(comm, class, sign_mode)?;
            self.processor.set_network()?;
            self.processor.set_show_instructions();
            introductory_screen::display(sign_mode)?;
        } else {
            self.processor.process_sign(comm, class, sign_mode)?;

            match sign_mode.decoding_mode() {
                DecodingMode::Auth => {
                    return if class != CommandClass::LastData {
                        Err(AppError::BadAuthSignSequence)
                    } else {
                        self.finalize_sign_auth(comm, sign_mode)
                    }
                }
                DecodingMode::PreAuth => {
                    return if class != CommandClass::LastData {
                        Err(AppError::BadSubintentSignSequence)
                    } else {
                        self.finalize_sign_si_hash(comm, sign_mode)
                    }
                }
                DecodingMode::Transaction => self.decode_intent(comm.get_data()?, class)?,
            }
        }

        if class == CommandClass::LastData {
            if sign_mode == SignMode::PreAuthRawEd25519
                || sign_mode == SignMode::PreAuthRawSecp256k1
            {
                self.finalize_sign_raw_si(comm, sign_mode)
            } else {
                self.finalize_sign_tx(comm, sign_mode)
            }
        } else {
            Ok(SignOutcome::SendNextPacket)
        }
    }

    fn finalize_sign_auth(
        &mut self,
        comm: &mut Comm,
        sign_mode: SignMode,
    ) -> Result<SignOutcome, AppError> {
        let value = comm.get_data()?;

        if value.len() < AUTH_MIN_VALID_LENGTH {
            return Err(AppError::BadAuthSignRequest);
        }

        let challenge = &value[..AUTH_CHALLENGE_LENGTH];
        let addr_start = AUTH_CHALLENGE_LENGTH + 1;
        let addr_end = addr_start + value[AUTH_CHALLENGE_LENGTH] as usize;
        let address = &value[addr_start..addr_end];
        let origin = &value[addr_end..];

        let mut nonce_hex = [0u8; AUTH_CHALLENGE_LENGTH * 2];

        Self::convert_to_hex_text(challenge, &mut nonce_hex);

        auth_details::display(address, origin, &nonce_hex);

        let rc = signature::ask_user(signature::SignType::Proof);

        if rc {
            let digest = self.processor.auth_digest(challenge, address, origin)?;
            self.processor
                .sign_message(comm, sign_mode, digest.as_bytes())
        } else {
            Ok(SignOutcome::SigningRejected)
        }
    }

    fn convert_to_hex_text(message: &[u8], message_hex: &mut [u8; 64]) {
        for (i, &byte) in message.iter().enumerate() {
            message_hex[i * 2] = upper_as_hex(byte);
            message_hex[i * 2 + 1] = lower_as_hex(byte);
        }
    }

    fn finalize_sign_tx(
        &mut self,
        comm: &mut Comm,
        sign_mode: SignMode,
    ) -> Result<SignOutcome, AppError> {
        let digest = self.processor.finalize()?;
        self.display_tx_info(sign_mode, &digest)?;

        let rc = signature::ask_user(signature::SignType::TX);

        if rc {
            self.processor
                .sign_message(comm, sign_mode, digest.as_bytes())
        } else {
            Ok(SignOutcome::SigningRejected)
        }
    }

    fn finalize_sign_si_hash(
        &mut self,
        comm: &mut Comm,
        sign_mode: SignMode,
    ) -> Result<SignOutcome, AppError> {
        if !(Settings::get().blind_signing) {
            return Err(AppError::BadSubintentSignState);
        }

        let message = comm.get_data()?;

        if message.len() != SUBINTENT_MESSAGE_LENGTH {
            return Err(AppError::BadSubintentSignRequest);
        }

        // The length is already checked above
        let digest = Digest(message.try_into().unwrap());
        self.finalize_sign_si(comm, sign_mode, message, &digest)
    }

    fn finalize_sign_raw_si(
        &mut self,
        comm: &mut Comm,
        sign_mode: SignMode,
    ) -> Result<SignOutcome, AppError> {
        let digest = self.processor.finalize()?;

        if !(Settings::get().blind_signing) {
            return Err(AppError::BadSubintentSignState);
        }
        
        self.finalize_sign_si(comm, sign_mode, digest.as_bytes(), &digest)
    }
    
    fn finalize_sign_si(
        &mut self,
        comm: &mut Comm,
        sign_mode: SignMode,
        message: &[u8],
        digest: &Digest
    ) -> Result<SignOutcome, AppError> {
        let mut message_hex = [0u8; SUBINTENT_MESSAGE_LENGTH * 2];
        Self::convert_to_hex_text(message, &mut message_hex);

        pre_auth_hash_details::display(&message_hex);

        let rc = signature::ask_user(signature::SignType::PreAuthHash);

        if rc {
            self.processor
                .sign_message(comm, sign_mode, digest.as_bytes())
        } else {
            Ok(SignOutcome::SigningRejected)
        }
    }

    fn display_transaction_fee(&mut self, detected_type: &DetectedTxType) {
        match detected_type {
            DetectedTxType::Transfer(details) => {
                if let Some(fee) = details.fee {
                    fee::display(&fee, &mut self.processor);
                }
            }
            DetectedTxType::Other(fee) | DetectedTxType::Error(fee) => match fee {
                Some(fee) => fee::display(fee, &mut self.processor),
                None => {}
            },
        }
    }

    fn display_tx_info(&mut self, sign_mode: SignMode, digest: &Digest) -> Result<(), AppError> {
        let detected_type = self.processor.get_detected_tx_type();

        match sign_mode {
            SignMode::TxEd25519Verbose | SignMode::TxSecp256k1Verbose => {
                self.display_transaction_fee(&detected_type);

                Ok(())
            }
            SignMode::TxEd25519Summary | SignMode::TxSecp256k1Summary => match detected_type {
                DetectedTxType::Transfer(details) => {
                    transfer::display(&details, &mut self.processor);

                    Ok(())
                }
                DetectedTxType::Other(fee) | DetectedTxType::Error(fee) => {
                    if Settings::get().blind_signing {
                        hash::display(digest, &fee, &mut self.processor);

                        Ok(())
                    } else {
                        hash::error();

                        Err(AppError::BadTxSignHashSignState)
                    }
                }
            },
            SignMode::AuthEd25519
            | SignMode::AuthSecp256k1
            | SignMode::PreAuthHashEd25519
            | SignMode::PreAuthHashSecp256k1
            | SignMode::PreAuthRawEd25519
            | SignMode::PreAuthRawSecp256k1 => Ok(()),
        }
    }

    fn decode_intent(&mut self, data: &[u8], class: CommandClass) -> Result<(), AppError> {
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
