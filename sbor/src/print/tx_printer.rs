use crate::bech32::network::NetworkId;
use crate::instruction::{Instruction, InstructionInfo};
use crate::instruction_extractor::{ExtractorEvent, InstructionHandler};
use crate::math::Decimal;
use crate::print::state::ParameterPrinterState;
use crate::print::tty::TTY;
use crate::print::tx_intent_type::TxIntentType;
use crate::sbor_decoder::SborEvent;
use crate::static_vec::StaticVec;
use crate::tx_features::{TxFeatures, TxType};
use crate::type_info::*;

// State transition chains:
// Lock_fee: CallMethod -> Address -> Name ("lock_fee") -> TupleLockFee -> (ValueLockFee) -> DoneLockFee
// Withdraw: CallMethod -> Address -> Name ("withdraw") -> TupleWithdraw -> (AddressWithdraw, ValueWithdraw) -> DoneWithdraw
// Deposit1: TakeFromWorktopByAmount -> ValueDeposit -> DoneDeposit1
// Deposit2: CallMethod -> Address -> Name ("deposit") -> DoneDeposit2

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum DecodingPhase {
    Dormant,

    CallMethod,
    Name,
    Address,

    TupleLockFee,
    ValueLockFee,
    DoneLockFee,

    TupleWithdraw,
    AddessWithdraw,
    ValueWithdraw,
    DoneWithdraw,

    TakeFromWorktopByAmount,
    ValueDeposit,
    AddressDeposit,
    DoneDeposit1,
    DoneDeposit2,

    DoneNothing,
}

#[derive(Copy, Clone, Debug)]
pub enum DetectedTxType {
    Transfer,
    TransferWithFee(Decimal),
    Other,
    OtherWithFee(Decimal),
    Error,
}

#[cfg(test)]
impl DetectedTxType {
    pub fn is_same(&self, other: &DetectedTxType) -> bool {
        match (self, other) {
            (DetectedTxType::Transfer, DetectedTxType::Transfer) => true,
            (DetectedTxType::Other, DetectedTxType::Other) => true,
            (DetectedTxType::TransferWithFee(fee), DetectedTxType::TransferWithFee(other_fee)) => {
                fee.is_same(&other_fee)
            }
            (DetectedTxType::OtherWithFee(fee), DetectedTxType::OtherWithFee(other_fee)) => {
                fee.is_same(&other_fee)
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
const MAX_TX_DATA_SIZE: usize = max(Decimal::SIZE_IN_BYTES, ADDRESS_LEN as usize);

#[derive(Copy, Clone)]
pub struct Address {
    address: [u8; ADDRESS_LEN as usize],
    is_set: bool,
}

impl Address {
    pub fn new() -> Self {
        Self {
            address: [0; ADDRESS_LEN as usize],
            is_set: false,
        }
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
        self.address = [0; ADDRESS_LEN as usize];
    }
}

pub struct TxIntentPrinter {
    network_id: NetworkId,
    intent_type: TxIntentType,
    decoding_phase: DecodingPhase,
    features: TxFeatures,
    data: StaticVec<u8, { MAX_TX_DATA_SIZE }>,
    fee: Decimal,
    amount: Decimal,
    tmp_address: Address,
    src_address: Address,
    dst_address: Address,
}

impl TxIntentPrinter {
    pub fn new(network_id: NetworkId) -> Self {
        Self {
            network_id,
            features: TxFeatures::new(),
            intent_type: TxIntentType::General,
            decoding_phase: DecodingPhase::Dormant,
            data: StaticVec::new(0),
            fee: Decimal::ZERO,
            amount: Decimal::ZERO,
            tmp_address: Address::new(),
            src_address: Address::new(),
            dst_address: Address::new(),
        }
    }

    pub fn set_intent_type(&mut self, intent_type: TxIntentType) {
        self.intent_type = intent_type;
    }

    pub fn reset(&mut self) {
        self.features.reset();
        self.intent_type = TxIntentType::General;
        self.network_id = NetworkId::LocalNet;
        self.decoding_phase = DecodingPhase::Dormant;
        self.data.clear();
        self.fee = Decimal::ZERO;
        self.amount = Decimal::ZERO;
        self.tmp_address.reset();
        self.src_address.reset();
        self.dst_address.reset();
    }

    pub fn set_network(&mut self, network_id: NetworkId) {
        self.network_id = network_id;
    }

    pub fn get_detected_tx_type(&self) -> DetectedTxType {
        match self.features.detected_type() {
            TxType::Transfer => DetectedTxType::Transfer,
            TxType::TransferWithFee => DetectedTxType::TransferWithFee(self.fee),
            TxType::Other => DetectedTxType::Other,
            TxType::OtherWithFee => DetectedTxType::OtherWithFee(self.fee),
            TxType::Error => DetectedTxType::Error,
        }
    }

    pub fn handle_with_state<T: Copy>(
        &mut self,
        event: ExtractorEvent,
        state: &mut ParameterPrinterState<T>,
    ) {
        match event {
            ExtractorEvent::InstructionStart(info, ..) => {
                self.start_instruction(info)
            }
            ExtractorEvent::ParameterStart(_, count, ..) => self.parameter_start(count),
            ExtractorEvent::ParameterData(data) => self.parameter_data(data),
            ExtractorEvent::ParameterEnd(..) => self.parameter_end(),
            ExtractorEvent::InstructionEnd => self.instruction_end(state),
            _ => self.handle_error(),
        };
    }

    fn start_instruction(&mut self, info: InstructionInfo) {
        self.decoding_phase = match info.instruction {
            Instruction::CallMethod => DecodingPhase::CallMethod,
            Instruction::TakeFromWorktopByAmount => DecodingPhase::TakeFromWorktopByAmount,
            _ => DecodingPhase::DoneNothing,
        };

        self.amount = Decimal::ZERO;
        self.tmp_address.reset();
    }

    fn instruction_end<T: Copy>(&mut self, state: &mut ParameterPrinterState<T>) {
        if self.intent_type != TxIntentType::Transfer {
            // So far we can decode/print only transfers
            return;
        }

        // Print intermediate state, if transaction is still Transfer or TransferWithFee
        if self.features.detected_type() == TxType::Transfer
            || self.features.detected_type() == TxType::TransferWithFee
        {
            match self.decoding_phase {
                DecodingPhase::DoneWithdraw => {
                    //TODO: display "From: <address>, Amount: <amount>"
                }
                DecodingPhase::DoneDeposit => {
                    //TODO: display "To: <address>, Amount: <amount>"
                }
                _ => {}
            }
        }
    }

    fn parameter_start(&mut self, param_count: u32) {
// State transition chains:
// Lock_fee: CallMethod -> Address -> Name ("lock_fee") -> TupleLockFee -> (ValueLockFee) -> DoneLockFee
// Withdraw: CallMethod -> Address -> Name ("withdraw") -> TupleWithdraw -> (AddressWithdraw, ValueWithdraw) -> DoneWithdraw
// Deposit1: TakeFromWorktopByAmount -> ValueDeposit -> DoneDeposit1
// Deposit2: CallMethod -> Address -> Name ("deposit") -> DoneDeposit2
        match (self.decoding_phase, param_count) {
            // CallMethod -> Address
            (DecodingPhase::CallMethod, 0) => self.decoding_phase = DecodingPhase::Address,
            // CallMethod -> Name
            (DecodingPhase::Address, 1) => self.decoding_phase = DecodingPhase::Name,

            // TakeFromWorktopByAmount -> ValueDeposit -> AddressDeposit
            (DecodingPhase::TakeFromWorktopByAmount, 0) => {
                self.decoding_phase = DecodingPhase::ValueDeposit
            }
            (DecodingPhase::ValueDeposit, 1) => self.decoding_phase = DecodingPhase::AddressDeposit,

            // TupleLockFee -> ValueLockFee
            (DecodingPhase::TupleLockFee, 0) => self.decoding_phase = DecodingPhase::ValueLockFee,

            // TupleWithdraw -> AddressWithdraw -> ValueWithdraw
            (DecodingPhase::TupleWithdraw, 0) => {
                self.decoding_phase = DecodingPhase::AddessWithdraw
            }
            (DecodingPhase::AddessWithdraw, 1) => {
                self.decoding_phase = DecodingPhase::ValueWithdraw
            }

            (_, _) => {}
        };
        self.data.clear();
    }

    fn parameter_data(&mut self, source_event: SborEvent) {
        match source_event {
            SborEvent::Data(data) => self.data.push(data),
            _ => {}
        }
    }

    fn extract_decimal(&mut self) -> Decimal {
        Decimal::try_from(self.data.as_slice()).unwrap_or(Decimal::ZERO)
    }

    fn extract_address(&mut self) {
        self.tmp_address.copy_from_slice(self.data.as_slice());
    }

    fn parameter_end(&mut self) {
        match self.decoding_phase {
            DecodingPhase::Name => {
                self.decoding_phase = match self.data.as_slice() {
                    b"lock_fee" => {
                        self.features.record_fee();
                        DecodingPhase::TupleLockFee
                    }
                    b"withdraw" => {
                        self.features.record_withdraw();
                        DecodingPhase::TupleWithdraw
                    }
                    b"deposit" => {
                        // Note that call method "deposit" we just record,
                        // but information is already displayed
                        self.features.record_deposit();
                        DecodingPhase::DoneNothing
                    }
                    _ => {
                        self.features.record_other();
                        DecodingPhase::DoneNothing
                    }
                }
            }

            DecodingPhase::ValueLockFee => {
                let amount = Decimal::try_from(self.data.as_slice()).unwrap_or(Decimal::ZERO);
                self.fee.accumulate(&amount);
                self.decoding_phase = DecodingPhase::DoneLockFee;
            }

            DecodingPhase::AddessWithdraw => {
                self.extract_address();
            }

            DecodingPhase::ValueWithdraw => {
                self.amount = self.extract_decimal();
                self.decoding_phase = DecodingPhase::DoneWithdraw;
            }

            DecodingPhase::ValueDeposit => {
                self.amount = self.extract_decimal();
            }

            DecodingPhase::AddressDeposit => {
                self.extract_address();
                self.decoding_phase = DecodingPhase::DoneDeposit;
            }

            _ => {}
        }
    }

    fn handle_error(&mut self) {
        self.features.record_error();
    }
}
