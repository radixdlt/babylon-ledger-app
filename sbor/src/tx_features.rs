pub struct TxFeatures(u8);

#[repr(u8)]
enum Feature {
    Withdraw = 0x01,
    Deposit = 0x02,
    Fee = 0x04,
    MixedTransaction = 0x08,
    TxError = 0x10,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum TxType {
    Transfer,
    TransferWithFee,
    Other,
    OtherWithFee,
    Error,
}

impl TxFeatures {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    pub fn record_deposit(&mut self) {
        if self.has(Feature::Deposit) {
            self.record_other();
        }

        self.0 |= Feature::Deposit as u8;
    }

    pub fn record_withdraw(&mut self) {
        self.0 |= Feature::Withdraw as u8;
    }

    pub fn record_fee(&mut self) {
        self.0 |= Feature::Fee as u8;
    }

    pub fn record_other(&mut self) {
        self.0 |= Feature::MixedTransaction as u8;
    }

    pub fn record_error(&mut self) {
        self.0 |= Feature::TxError as u8;
    }

    pub fn detected_type(&self) -> TxType {
        if self.has(Feature::TxError) {
            return TxType::Error;
        }

        if self.has(Feature::Withdraw) && self.has(Feature::Deposit) {
            if self.has(Feature::Fee) {
                TxType::TransferWithFee
            } else {
                TxType::Transfer
            }
        } else {
            if self.has(Feature::Fee) {
                TxType::OtherWithFee
            } else {
                TxType::Other
            }
        }
    }

    fn has(&self, feature: Feature) -> bool {
        self.0 & feature as u8 != 0
    }
}
