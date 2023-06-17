use crate::bech32::encoder::*;
use crate::bech32::hrp::*;
use crate::bech32::network::NetworkId;
use crate::instruction::{Instruction, InstructionInfo};
use crate::instruction_extractor::ExtractorEvent;
use crate::math::Decimal;
use crate::print::tx_intent_type::TxIntentType;
use crate::sbor_decoder::SborEvent;
use crate::static_vec::StaticVec;
use crate::type_info::*;

// Lock_fee: CallMethod -> Address -> Name ("lock_fee") -> TupleLockFee -> (ValueLockFee) -> DoneLockFee
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum FeePhase {
    Start,
    CallMethod,
    Address,
    Name,
    Tuple,
    Value,
}

// State transition chains:
// Withdraw: CallMethod -> Address -> Name ("withdraw") -> TupleWithdraw -> (AddressWithdraw..)
// Deposit1: TakeFromWorktopByAmount -> ValueDeposit -> DoneDeposit1
// Deposit2: CallMethod -> Address -> Name ("deposit") -> DoneDeposit2

// Summary:
// Start -> CallMethod -> AddressWithdraw -> ExpectWithdraw + ("withdraw") ->
// /- WithdrawDone (+ TakeFromWorktopByAmount) -> ValueDeposit -> Resource -> ResourceDone + end of instruction ->
// \- WithdrawDone (+ TakeFromWorktopByIds) -> ValueCountDeposit -> ValueCountDepositIds -> ValueCountDone -> Resource -> ResourceDone + end of instruction ->
// ExpectDepositCall (+ CallMethod) -> AddressDeposit -> ExpectDeposit + ("deposit") -> DoneTransfer

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum DecodingPhase {
    Start,
    CallMethod,
    AddressWithdraw,
    ExpectWithdraw,
    WithdrawDone,
    ValueCountDeposit,
    ValueCountDepositIds,
    ValueCountDone,
    ValueDeposit,
    Resource,
    ResourceDone,
    ExpectDepositCall,
    AddressDeposit,
    ExpectDeposit,
    DoneTransfer,
    NonConformingTransaction,
    DecodingError,
}

#[derive(Copy, Clone, Debug)]
pub enum DetectedTxType {
    Transfer {
        fee: Option<Decimal>,
        src_address: Address,
        dst_address: Address,
        res_address: Address,
        amount: Decimal,
    },
    Other(Option<Decimal>),
    Error(Option<Decimal>),
}

#[cfg(test)]
impl DetectedTxType {
    pub fn is_same(&self, other: &DetectedTxType) -> bool {
        match (self, other) {
            (
                DetectedTxType::Transfer {
                    fee,
                    src_address,
                    dst_address,
                    res_address,
                    amount,
                },
                DetectedTxType::Transfer {
                    fee: other_fee,
                    src_address: other_src_address,
                    dst_address: other_dst_address,
                    res_address: other_res_address,
                    amount: other_amount,
                },
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

#[inline(always)]
const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

//Max of address length and decimal length
const MAX_TX_DATA_SIZE: usize = max(Decimal::SIZE_IN_BYTES, ADDRESS_STATIC_LEN as usize);

#[derive(Copy, Clone, Debug)]
pub struct Address {
    address: [u8; ADDRESS_STATIC_LEN as usize],
    is_set: bool,
}

impl Address {
    pub fn new() -> Self {
        Self {
            address: [0; ADDRESS_STATIC_LEN as usize],
            is_set: false,
        }
    }

    pub fn from_array(src: [u8; ADDRESS_STATIC_LEN as usize]) -> Self {
        Self {
            address: src,
            is_set: true,
        }
    }

    pub fn as_ref(&self) -> &[u8] {
        &self.address
    }

    pub fn is_set(&self) -> bool {
        self.is_set
    }

    pub fn is_xrd(&self) -> bool {
        if !self.is_set {
            return false;
        }

        if self.address[0] != 0x01 {
            return false;
        }

        for i in 1..ADDRESS_STATIC_LEN as usize {
            if self.address[i] != 0x00 {
                return false;
            }
        }

        true
    }

    pub fn copy_from_slice(&mut self, src: &[u8]) {
        self.address.copy_from_slice(src);
        self.is_set = true;
    }

    pub fn copy_from_other(&mut self, other: &Address) {
        self.is_set = other.is_set;

        if self.is_set {
            self.address.copy_from_slice(&other.address);
        }
    }

    pub fn reset(&mut self) {
        self.is_set = false;
        self.address = [0; ADDRESS_STATIC_LEN as usize];
    }

    pub fn is_same(&self, other: &Address) -> bool {
        if self.is_set && other.is_set {
            self.address == other.address
        } else {
            false
        }
    }

    pub fn prefix(&self) -> Option<&'static str> {
        if self.is_set {
            hrp_prefix(self.address[0])
        } else {
            None
        }
    }

    pub fn format<const N: usize>(&self, data: &mut StaticVec<u8, N>, network_id: NetworkId) {
        match self.prefix() {
            Some(prefix) => {
                data.extend_from_slice(prefix.as_bytes());
                data.extend_from_slice(hrp_suffix(network_id).as_bytes());

                let encoding_result = Bech32::encode(data.as_slice(), self.as_ref());
                data.clear();

                match encoding_result {
                    Ok(encoder) => data.extend_from_slice(encoder.encoded()),
                    Err(..) => data.extend_from_slice(b"<bech32 error>"), // unlikely, just for completeness
                }
            }
            None => data.extend_from_slice(b"unknown address type"),
        }
    }
}

pub struct TxIntentPrinter {
    intent_type: TxIntentType,
    decoding_phase: DecodingPhase,
    fee_phase: FeePhase,
    data: StaticVec<u8, { MAX_TX_DATA_SIZE }>,
    fee: Decimal,
    amount: Decimal,
    src_address: Address,
    dst_address: Address,
    res_address: Address,
}

impl TxIntentPrinter {
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
            DecodingPhase::DoneTransfer => DetectedTxType::Transfer {
                fee,
                src_address: self.src_address,
                dst_address: self.dst_address,
                res_address: self.res_address,
                amount: self.amount,
            },
            DecodingPhase::DecodingError => DetectedTxType::Error(fee),
            _ => DetectedTxType::Other(fee),
        }
    }

    pub fn handle(&mut self, event: ExtractorEvent) {
        match event {
            ExtractorEvent::InstructionStart(info, ..) => self.instruction_start(info),
            ExtractorEvent::ParameterStart(_, count, ..) => self.parameter_start(count),
            ExtractorEvent::ParameterData(data) => self.parameter_data(data),
            ExtractorEvent::ParameterEnd(..) => self.parameter_end(),
            ExtractorEvent::InstructionEnd => self.instruction_end(),
            _ => self.handle_error(),
        };
    }

    fn instruction_start(&mut self, info: InstructionInfo) {
        match (self.fee_phase, info.instruction) {
            (FeePhase::Start, Instruction::CallMethod) => {
                self.fee_phase = FeePhase::Address;
            }
            (_, _) => {}
        }

        if self.intent_type != TxIntentType::Transfer {
            return;
        }

        match (self.decoding_phase, info.instruction) {
            (DecodingPhase::Start, Instruction::CallMethod) => {
                self.decoding_phase = DecodingPhase::CallMethod;
            }
            (DecodingPhase::WithdrawDone, Instruction::TakeFromWorktop) => {
                self.decoding_phase = DecodingPhase::ValueDeposit;
            }
            (DecodingPhase::WithdrawDone, Instruction::TakeNonFungiblesFromWorktop) => {
                self.decoding_phase = DecodingPhase::ValueCountDeposit;
            }
            (DecodingPhase::ExpectDepositCall, Instruction::CallMethod) => {
                self.decoding_phase = DecodingPhase::AddressDeposit;
            }
            // TODO: How to reliably detect nonconforming transaction here?
            (_, _) => {}
        }
    }

    fn instruction_end(&mut self) {
        match self.decoding_phase {
            DecodingPhase::ResourceDone => {
                self.decoding_phase = DecodingPhase::ExpectDepositCall;
            }
            _ => {}
        }
        self.fee_phase = FeePhase::Start;
    }

    fn parameter_start(&mut self, param_count: u32) {
        self.data.clear();

        match (self.decoding_phase, param_count) {
            (DecodingPhase::CallMethod, 0) => {
                self.decoding_phase = DecodingPhase::AddressWithdraw;
            }
            (DecodingPhase::AddressWithdraw, 1) => {
                self.decoding_phase = DecodingPhase::ExpectWithdraw;
            }
            (DecodingPhase::ValueCountDeposit, 0) => {
                self.decoding_phase = DecodingPhase::ValueCountDepositIds;
            }
            (DecodingPhase::ExpectDepositCall, 0) => {
                self.decoding_phase = DecodingPhase::AddressDeposit;
            }
            (DecodingPhase::ValueCountDone, 1) => {
                self.decoding_phase = DecodingPhase::Resource;
            }

            (_, _) => {}
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
            SborEvent::Len(len) if self.decoding_phase == DecodingPhase::ValueCountDepositIds => {
                self.amount = Decimal::whole(len.into());
                self.decoding_phase = DecodingPhase::ValueCountDone;
            }
            _ => {}
        }
    }

    fn extract_decimal(&mut self) -> Decimal {
        Decimal::try_from(self.data.as_slice()).unwrap_or(Decimal::ZERO)
    }

    fn parameter_end(&mut self) {
        match self.decoding_phase {
            DecodingPhase::ExpectWithdraw => {
                if self.data.as_slice() == b"withdraw" {
                    self.decoding_phase = DecodingPhase::WithdrawDone;
                } else if self.data.as_slice() == b"withdraw_non_fungibles" {
                    self.decoding_phase = DecodingPhase::WithdrawDone;
                } else {
                    // Restart decoding
                    self.decoding_phase = DecodingPhase::Start;
                }
            }

            DecodingPhase::ExpectDeposit => {
                if self.data.as_slice() == b"deposit" {
                    self.decoding_phase = DecodingPhase::DoneTransfer;
                } else {
                    self.decoding_phase = DecodingPhase::NonConformingTransaction;
                }
            }

            DecodingPhase::AddressWithdraw => {
                self.src_address.copy_from_slice(self.data.as_slice());
                self.decoding_phase = DecodingPhase::ExpectWithdraw;
            }

            DecodingPhase::ValueDeposit => {
                self.amount = self.extract_decimal();
                self.decoding_phase = DecodingPhase::Resource;
            }

            DecodingPhase::Resource => {
                self.res_address.copy_from_slice(self.data.as_slice());
                self.decoding_phase = DecodingPhase::ResourceDone;
            }

            DecodingPhase::AddressDeposit => {
                self.dst_address.copy_from_slice(self.data.as_slice());
                self.decoding_phase = DecodingPhase::ExpectDeposit;
            }

            _ => {}
        }

        match self.fee_phase {
            FeePhase::Name => {
                if self.data.as_slice() == b"lock_fee" {
                    self.fee_phase = FeePhase::Value;
                } else {
                    self.fee_phase = FeePhase::Start;
                }
            }
            FeePhase::Value => {
                let fee = self.extract_decimal();

                self.fee.accumulate(&fee);
                self.fee_phase = FeePhase::Start;
            }
            _ => {}
        }
    }

    fn handle_error(&mut self) {
        self.decoding_phase = DecodingPhase::DecodingError;
    }
}
