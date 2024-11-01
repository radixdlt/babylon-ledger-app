/// Transaction summary detector is used to determine the type of the transaction intent and collect
/// information about fees.
/// Implementation consists of two independent state machines - one for detecting the intent type and
/// other to collect fee information. Both of them use information about decoded instructions
/// received from `InstructionExtractor`.
use crate::bech32::address::Address;
use crate::instruction::{Instruction, InstructionInfo};
use crate::instruction_extractor::ExtractorEvent;
use crate::math::Decimal;
use crate::print::tx_intent_type::TxIntentType;
use crate::sbor_decoder::SborEvent;
use crate::static_vec::StaticVec;
use crate::type_info::*;

/// Transaction fee collector state machine phases.
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum FeePhase {
    Start,
    CallMethod,
    Address,
    Name,
    Tuple,
    Value,
    ValueStart,
}

/// Transaction type detector state machine phases.
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum DecodingPhase {
    Start,
    CallMethod,
    AddressWithdraw,
    ExpectWithdraw,
    WithdrawDone,
    ValueDepositCount, //Outer array
    ValueDepositCountIds,
    Resource,
    NonFungibleResource,
    ValueDeposit,
    ValueDepositDone,
    ExpectDepositCall,
    ExpectAddressDeposit,
    AddressDeposit,
    ExpectDeposit,
    DoneTransfer,
    NonConformingTransaction,
    DecodingError,
}

#[derive(Copy, Clone, Debug)]
pub struct TransferDetails {
    pub fee: Option<Decimal>,
    pub src_address: Address, // From ...
    pub dst_address: Address, // To ...
    pub res_address: Address, // Resource ...
    pub amount: Decimal,      // Amount ...
}

#[derive(Copy, Clone, Debug)]
pub enum DetectedTxType {
    Transfer(TransferDetails),
    Other(Option<Decimal>),
    Error(Option<Decimal>),
}

trait SameString {
    fn eq(&self, other: &[u8]) -> bool;
}

impl SameString for StaticVec<u8, { MAX_TX_DATA_SIZE }> {
    fn eq(&self, other: &[u8]) -> bool {
        let data = self.as_slice();

        if data.len() != other.len() {
            return false;
        }
        for i in 0..data.len() {
            if data[i] != other[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
impl DetectedTxType {
    pub fn is_same(&self, other: &DetectedTxType) -> bool {
        match (self, other) {
            (
                DetectedTxType::Transfer(TransferDetails {
                    fee,
                    src_address,
                    dst_address,
                    res_address,
                    amount,
                }),
                DetectedTxType::Transfer(TransferDetails {
                    fee: other_fee,
                    src_address: other_src_address,
                    dst_address: other_dst_address,
                    res_address: other_res_address,
                    amount: other_amount,
                }),
            ) => {
                let fee_match = match (fee, other_fee) {
                    (None, None) => true,
                    (Some(a), Some(b)) => a.is_same(&b),
                    _ => false,
                };
                return fee_match
                    && src_address.is_same(other_src_address)
                    && dst_address.is_same(other_dst_address)
                    && res_address.is_same(other_res_address)
                    && amount.is_same(other_amount);
            }

            (DetectedTxType::Other(fee), DetectedTxType::Other(other_fee)) => {
                match (fee, other_fee) {
                    (None, None) => true,
                    (Some(a), Some(b)) => a.is_same(&b),
                    _ => false,
                }
            }

            (DetectedTxType::Error(fee), DetectedTxType::Error(other_fee)) => {
                match (fee, other_fee) {
                    (None, None) => true,
                    (Some(a), Some(b)) => a.is_same(&b),
                    _ => false,
                }
            }

            _ => false,
        }
    }
}

/// Size of temporary buffer for parameter data. It should be enough to store any parameter data
/// which we're going to use. So far we're operating with addresses, decimal numbers and fixed
/// method names. None of them exceeds 40 bytes.
const MAX_TX_DATA_SIZE: usize = 40;

pub struct TxSummaryDetector {
    intent_type: TxIntentType,
    decoding_phase: DecodingPhase,
    fee_phase: FeePhase,
    data: StaticVec<u8, { MAX_TX_DATA_SIZE }>, // Temporary buffer for parameter data
    fee: Decimal,
    amount: Decimal,
    src_address: Address,
    dst_address: Address,
    res_address: Address,
}

impl TxSummaryDetector {
    pub fn new() -> Self {
        Self {
            intent_type: TxIntentType::General,
            decoding_phase: DecodingPhase::Start,
            fee_phase: FeePhase::Start,
            data: StaticVec::new(0),
            fee: Decimal::ZERO,
            amount: Decimal::ZERO,
            src_address: Address::new(),
            dst_address: Address::new(),
            res_address: Address::new(),
        }
    }

    pub fn set_intent_type(&mut self, intent_type: TxIntentType) {
        self.intent_type = intent_type;
    }

    pub fn reset(&mut self) {
        self.intent_type = TxIntentType::General;
        self.decoding_phase = DecodingPhase::Start;
        self.fee_phase = FeePhase::Start;
        self.data.clear();
        self.fee = Decimal::ZERO;
        self.amount = Decimal::ZERO;
        self.src_address.reset();
        self.dst_address.reset();
        self.res_address.reset();
    }

    pub fn get_detected_tx_type(&self) -> DetectedTxType {
        let fee = if self.fee.is_same(&Decimal::ZERO) {
            None
        } else {
            Some(self.fee)
        };

        if self.intent_type != TxIntentType::Transfer {
            return DetectedTxType::Other(fee);
        }

        if !(self.src_address.is_set() && self.dst_address.is_set() && self.res_address.is_set()) {
            return DetectedTxType::Error(fee);
        }

        match self.decoding_phase {
            DecodingPhase::DoneTransfer => DetectedTxType::Transfer(TransferDetails {
                fee: fee,
                src_address: self.src_address,
                dst_address: self.dst_address,
                res_address: self.res_address,
                amount: self.amount,
            }),
            DecodingPhase::DecodingError => DetectedTxType::Error(fee),
            _ => DetectedTxType::Other(fee),
        }
    }

    pub fn handle(&mut self, event: ExtractorEvent) {
        match event {
            ExtractorEvent::InstructionStart(info, ..) => self.instruction_start(info),
            ExtractorEvent::ParameterStart(event, count, ..) => self.parameter_start(event, count),
            ExtractorEvent::ParameterData(data) => self.parameter_data(data),
            ExtractorEvent::ParameterEnd(..) => self.parameter_end(),
            ExtractorEvent::InstructionEnd => self.instruction_end(),
            _ => self.decoding_phase = DecodingPhase::DecodingError,
        };
    }

    fn instruction_start(&mut self, info: InstructionInfo) {
        match (self.fee_phase, info.instruction) {
            (FeePhase::Start, Instruction::CallMethod) => {
                self.fee_phase = FeePhase::Address;
            }
            (_, _) => {
                self.fee_phase = FeePhase::Start;
            }
        }

        if self.intent_type != TxIntentType::Transfer {
            return;
        }

        match (self.decoding_phase, info.instruction) {
            (DecodingPhase::Start, Instruction::CallMethod) => {
                self.decoding_phase = DecodingPhase::CallMethod;
            }
            (DecodingPhase::WithdrawDone, Instruction::TakeFromWorktop) => {
                self.decoding_phase = DecodingPhase::Resource;
            }
            (DecodingPhase::WithdrawDone, Instruction::TakeNonFungiblesFromWorktop) => {
                self.decoding_phase = DecodingPhase::NonFungibleResource;
            }
            (DecodingPhase::ExpectDepositCall, Instruction::CallMethod) => {
                self.decoding_phase = DecodingPhase::ExpectAddressDeposit;
            }
            (DecodingPhase::DoneTransfer, _) => {
                if info.instruction != Instruction::CallMethod {
                    self.decoding_phase = DecodingPhase::NonConformingTransaction;
                }
            }
            (_, _) => {
                self.decoding_phase = DecodingPhase::NonConformingTransaction;
            }
        }
    }

    fn instruction_end(&mut self) {
        match self.decoding_phase {
            DecodingPhase::ValueDepositDone => {
                self.decoding_phase = DecodingPhase::ExpectDepositCall;
            }
            _ => {}
        }
        self.fee_phase = FeePhase::Start;
    }

    fn parameter_start(&mut self, event: SborEvent, param_count: u32) {
        self.data.clear();

        if let SborEvent::Start { type_id, .. } = event {
            match (self.decoding_phase, param_count, type_id) {
                (DecodingPhase::CallMethod, 0, TYPE_ADDRESS) => {
                    self.decoding_phase = DecodingPhase::AddressWithdraw;
                }
                (DecodingPhase::AddressWithdraw, 1, TYPE_STRING) => {
                    self.decoding_phase = DecodingPhase::ExpectWithdraw;
                }
                (DecodingPhase::ExpectAddressDeposit, 0, TYPE_ADDRESS) => {
                    self.decoding_phase = DecodingPhase::AddressDeposit;
                }
                (DecodingPhase::ValueDepositCount, 1, TYPE_ARRAY) => {
                    self.decoding_phase = DecodingPhase::ValueDepositCountIds;
                }

                (_, _, _) => {}
            }
        };

        match (self.fee_phase, param_count) {
            (FeePhase::Address, 1) => {
                self.fee_phase = FeePhase::Name;
            }
            (FeePhase::Tuple, 0) => {
                self.fee_phase = FeePhase::Value;
            }
            (_, _) => {}
        }
    }

    fn parameter_data(&mut self, source_event: SborEvent) {
        match source_event {
            SborEvent::Data(data) => self.data.push(data),
            SborEvent::Len(len) if self.decoding_phase == DecodingPhase::ValueDepositCountIds => {
                self.amount = Decimal::whole(len.into());
                self.decoding_phase = DecodingPhase::ValueDepositDone;
            }
            SborEvent::Start { type_id, .. } if self.fee_phase == FeePhase::ValueStart => {
                if type_id == TYPE_DECIMAL {
                    self.data.clear();
                    self.fee_phase = FeePhase::Value;
                }
            }
            SborEvent::End { type_id, .. } if self.fee_phase == FeePhase::Value => {
                if type_id == TYPE_DECIMAL {
                    let fee = self.extract_decimal();
                    self.fee.accumulate(&fee);
                    self.fee_phase = FeePhase::Start;
                }
            }
            _ => {}
        }
    }

    fn extract_decimal(&mut self) -> Decimal {
        Decimal::try_from(self.data.as_slice()).unwrap_or(Decimal::ZERO)
    }

    fn parameter_end(&mut self) {
        match self.decoding_phase {
            DecodingPhase::AddressWithdraw => {
                if self.data.len() == ADDRESS_STATIC_LEN as usize {
                    self.src_address.copy_from_slice(self.data.as_slice());
                    self.decoding_phase = DecodingPhase::ExpectWithdraw;
                } else {
                    self.decoding_phase = DecodingPhase::DecodingError;
                }
            }

            DecodingPhase::AddressDeposit => {
                if self.data.len() == ADDRESS_STATIC_LEN as usize {
                    self.dst_address.copy_from_slice(self.data.as_slice());
                    self.decoding_phase = DecodingPhase::ExpectDeposit;
                } else {
                    self.decoding_phase = DecodingPhase::DecodingError;
                }
            }

            DecodingPhase::ExpectWithdraw => {
                if self.data.eq(b"withdraw")
                    || self.data.eq(b"withdraw_non_fungibles")
                    || self.data.eq(b"lock_fee_and_withdraw")
                    || self.data.eq(b"lock_fee_and_withdraw_non_fungibles")
                {
                    self.decoding_phase = DecodingPhase::WithdrawDone;
                } else {
                    // Restart decoding
                    self.decoding_phase = DecodingPhase::Start;
                }
            }

            DecodingPhase::ExpectDeposit => {
                if self.data.eq(b"deposit")
                    || self.data.eq(b"try_deposit_or_abort")
                    || self.data.eq(b"try_deposit_or_refund")
                {
                    self.decoding_phase = DecodingPhase::DoneTransfer;
                    self.fee_phase = FeePhase::Start;
                    return;
                } else {
                    self.decoding_phase = DecodingPhase::NonConformingTransaction;
                }
            }

            DecodingPhase::ValueDeposit => {
                if self.data.len() == DECIMAL_LEN as usize {
                    self.amount = self.extract_decimal();
                    self.decoding_phase = DecodingPhase::ValueDepositDone;
                } else {
                    self.decoding_phase = DecodingPhase::DecodingError;
                }
            }

            DecodingPhase::Resource => {
                if self.data.len() == ADDRESS_STATIC_LEN as usize {
                    self.res_address.copy_from_slice(self.data.as_slice());
                    self.decoding_phase = DecodingPhase::ValueDeposit;
                } else {
                    self.decoding_phase = DecodingPhase::DecodingError;
                }
            }

            DecodingPhase::NonFungibleResource => {
                if self.data.len() == ADDRESS_STATIC_LEN as usize {
                    self.res_address.copy_from_slice(self.data.as_slice());
                    self.decoding_phase = DecodingPhase::ValueDepositCount;
                } else {
                    self.decoding_phase = DecodingPhase::DecodingError;
                }
            }

            _ => {}
        }

        match self.fee_phase {
            FeePhase::Name => {
                if self.data.eq(b"lock_fee") {
                    self.fee_phase = FeePhase::Value;
                } else if self.data.eq(b"lock_fee_and_withdraw")
                    || self.data.eq(b"lock_fee_and_withdraw_non_fungibles")
                {
                    self.fee_phase = FeePhase::ValueStart;
                } else {
                    self.fee_phase = FeePhase::Start;

                    if self.decoding_phase == DecodingPhase::DoneTransfer {
                        self.decoding_phase = DecodingPhase::NonConformingTransaction;
                    }
                }
            }
            FeePhase::Value => {
                if self.data.len() == DECIMAL_LEN as usize {
                    let fee = self.extract_decimal();
                    self.fee.accumulate(&fee);
                } else {
                    self.decoding_phase = DecodingPhase::DecodingError;
                }
                self.fee_phase = FeePhase::Start;
            }
            _ => {}
        }
    }
}
