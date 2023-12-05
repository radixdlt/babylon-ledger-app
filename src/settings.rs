use ledger_device_sdk::nvm::{AtomicStorage, SingleStorage};
use ledger_device_sdk::NVMData;

const BIT_VERBOSE_MODE: u32 = 0x01;
const BIT_HASH_SIGN: u32 = 0x02;

// Note that bits are stored in inverse mode (0 = true, 1 = false)
#[link_section = ".nvm_data"]
static mut SETTINGS: NVMData<AtomicStorage<u32>> = NVMData::new(AtomicStorage::new(&3));

pub struct Settings {
    pub verbose_mode: bool,
    pub blind_signing: bool,
}

impl Settings {
    pub fn get() -> Self {
        let settings = unsafe { SETTINGS.get_mut() };
        let value = *settings.get_ref();

        Settings {
            verbose_mode: (value & BIT_VERBOSE_MODE) == 0,
            blind_signing: (value & BIT_HASH_SIGN) == 0,
        }
    }

    pub fn update(&self) {
        let settings = unsafe { SETTINGS.get_mut() };
        let value = (!self.verbose_mode as u32 * BIT_VERBOSE_MODE)
            | (!self.blind_signing as u32 * BIT_HASH_SIGN);

        if value != *settings.get_ref() {
            settings.update(&value);
        }
    }

    pub fn as_bytes(&self) -> [u8; 2] {
        [
            [0x00, 0x01][self.verbose_mode as usize],
            [0x00, 0x01][self.blind_signing as usize],
        ]
    }
}
