// Suppress warnings about StatusWords variants not being used
#![allow(dead_code)]
use core::convert::TryFrom;

use ledger_device_sdk::seph;
use ledger_device_sdk::sys::buttons::{get_button_event, ButtonEvent, ButtonsState};
use ledger_device_sdk::sys::seph as sys_seph;
pub use ledger_device_sdk::sys::BOLOS_UX_CONTINUE;
pub use ledger_device_sdk::sys::BOLOS_UX_IGNORE;
pub use ledger_device_sdk::sys::BOLOS_UX_OK;
use ledger_device_sdk::sys::*;

use crate::app_error::AppError;
use crate::command::Command;

unsafe extern "C" {
    pub unsafe static mut G_ux_params: bolos_ux_params_t;
}

// These codes are from the Ledger SDK, so we ought to support them even if they are not used
#[derive(Copy, Clone)]
#[repr(u16)]
pub enum StatusWords {
    Ok = 0x9000,
    NothingReceived = 0x6982,
    BadCla = 0x6e00,
    BadIns = 0x6e01,
    BadP1P2 = 0x6e02,
    BadLen = 0x6e03,
    UserCancelled = 0x6e04,
    Unknown = 0x6d00,
    Panic = 0xe000,
    DeviceLocked = 0x5515,
}

#[derive(Debug)]
#[repr(u8)]
pub enum SyscallError {
    InvalidParameter = 2,
    Overflow,
    Security,
    InvalidCrc,
    InvalidChecksum,
    InvalidCounter,
    NotSupported,
    InvalidState,
    Timeout,
    Unspecified,
}

impl From<u32> for SyscallError {
    fn from(e: u32) -> SyscallError {
        match e {
            2 => SyscallError::InvalidParameter,
            3 => SyscallError::Overflow,
            4 => SyscallError::Security,
            5 => SyscallError::InvalidCrc,
            6 => SyscallError::InvalidChecksum,
            7 => SyscallError::InvalidCounter,
            8 => SyscallError::NotSupported,
            9 => SyscallError::InvalidState,
            10 => SyscallError::Timeout,
            _ => SyscallError::Unspecified,
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Reply(pub u16);

impl From<StatusWords> for Reply {
    fn from(sw: StatusWords) -> Reply {
        Reply(sw as u16)
    }
}

impl From<SyscallError> for Reply {
    fn from(exc: SyscallError) -> Reply {
        Reply(0x6800 + exc as u16)
    }
}

extern "C" {
    pub fn io_usb_hid_send(
        sndfct: unsafe extern "C" fn(*mut u8, u16),
        sndlength: u16,
        apdu_buffer: *const u8,
    );
}

#[derive(Eq, PartialEq)]
pub enum Event<T> {
    /// APDU event
    Command(T),
    /// Button press or release event
    Button(ButtonEvent),
    /// Ticker
    Ticker,
}

pub struct Comm {
    pub apdu_buffer: [u8; 272],
    pub rx: usize,
    pub tx: usize,
    buttons: ButtonsState,

    pub apdu_type: u8,
    pub work_buffer: [u8; 273],
}

impl Default for Comm {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ApduHeader {
    /// Class
    pub cla: u8,
    /// Instruction
    pub ins: u8,
    /// Parameter 1
    pub p1: u8,
    /// Parameter 2
    pub p2: u8,
}

impl Comm {
    pub const fn new() -> Self {
        Self {
            apdu_buffer: [0u8; 272],
            rx: 0,
            tx: 0,
            buttons: ButtonsState::new(),
            apdu_type: seph::PacketTypes::PacketTypeNone as u8,
            work_buffer: [0u8; 273],
        }
    }

    fn apdu_send(&mut self) {
        sys_seph::io_tx(self.apdu_type, &self.apdu_buffer, self.tx);
        self.tx = 0;
    }

    pub fn next_event<T: TryFrom<ApduHeader>>(&mut self) -> Event<T> {
        loop {
            if let Some(value) = self.read_event() {
                return value;
            }
        }
    }

    pub fn read_event<T: TryFrom<ApduHeader>>(&mut self) -> Option<Event<T>> {
        // Fetch the next message from the MCU
        let length = sys_seph::io_rx(&mut self.work_buffer, true);
        if length <= 0 {
            return None;
        }

        let packet_type = self.work_buffer[0];

        match seph::PacketTypes::from(packet_type) {
            seph::PacketTypes::PacketTypeSeph | seph::PacketTypes::PacketTypeSeEvent => {
                // SE or SEPH event
                let mut seph_buffer = [0u8; 272];
                seph_buffer[0..272].copy_from_slice(&self.work_buffer[1..273]);

                let tag = seph_buffer[0];
                let _len: usize = u16::from_be_bytes([seph_buffer[1], seph_buffer[2]]) as usize;

                match seph::Events::from(tag) {
                    seph::Events::ButtonPushEvent => {
                        let button_info = seph_buffer[3] >> 1;
                        if let Some(btn_evt) = get_button_event(&mut self.buttons, button_info) {
                            return Some(Event::Button(btn_evt));
                        }
                    }

                    seph::Events::TickerEvent => return Some(Event::Ticker),

                    seph::Events::ItcEvent => {
                        #[cfg(target_os = "nanox")]
                        match seph::ItcUxEvent::from(seph_buffer[3]) {
                            seph::ItcUxEvent::AskBlePairing => unsafe {
                                G_ux_params.ux_id = BOLOS_UX_ASYNCHMODAL_PAIRING_REQUEST;
                                G_ux_params.len = 20;
                                G_ux_params.u.pairing_request.type_ = seph_buffer[4];
                                G_ux_params.u.pairing_request.pairing_info_len = (_len - 2) as u32;
                                #[allow(clippy::manual_memcpy)]
                                for i in 0..G_ux_params.u.pairing_request.pairing_info_len as usize
                                {
                                    G_ux_params.u.pairing_request.pairing_info[i] =
                                        seph_buffer[5 + i];
                                }
                                G_ux_params.u.pairing_request.pairing_info
                                    [G_ux_params.u.pairing_request.pairing_info_len as usize] = 0;
                                os_ux(&raw mut G_ux_params as *mut bolos_ux_params_t);
                            },

                            seph::ItcUxEvent::BlePairingStatus => unsafe {
                                G_ux_params.ux_id = BOLOS_UX_ASYNCHMODAL_PAIRING_STATUS;
                                G_ux_params.len = 0;
                                G_ux_params.u.pairing_status.pairing_ok = seph_buffer[4];
                                os_ux(&raw mut G_ux_params as *mut bolos_ux_params_t);
                            },

                            seph::ItcUxEvent::Redisplay => {
                                #[cfg(feature = "nano_nbgl")]
                                unsafe {
                                    nbgl_objAllowDrawing(true);
                                    nbgl_screenRedraw();
                                    nbgl_refresh();
                                }
                            }

                            _ => return None,
                        }
                        return None;
                    }

                    _ => {
                        if !cfg!(feature = "nano_nbgl") {
                            unsafe {
                                G_ux_params.ux_id = BOLOS_UX_EVENT;
                                G_ux_params.len = 0;
                                os_ux(&raw mut G_ux_params as *mut bolos_ux_params_t);
                            }
                        } else {
                            #[cfg(feature = "nano_nbgl")]
                            unsafe {
                                ux_process_default_event();
                            }
                        }
                    }
                }
            }

            seph::PacketTypes::PacketTypeRawApdu
            | seph::PacketTypes::PacketTypeUsbHidApdu
            | seph::PacketTypes::PacketTypeUsbWebusbApdu
            | seph::PacketTypes::PacketTypeBleApdu => {
                unsafe {
                    if os_perso_is_pin_set() == BOLOS_TRUE.try_into().unwrap()
                        && os_global_pin_is_validated() != BOLOS_TRUE.try_into().unwrap()
                    {
                        self.reply(StatusWords::DeviceLocked);
                        return None;
                    }
                }
                self.apdu_buffer[0..272].copy_from_slice(&self.work_buffer[1..273]);
                self.apdu_type = packet_type;
                self.rx = (length - 1) as usize;
                // Reject incomplete APDUs
                if self.rx < 4 {
                    self.reply(StatusWords::BadLen);
                    return None;
                }

                // Check for data length by using `get_data`
                if let Err(sw) = self.get_data() {
                    self.reply(sw);
                    return None;
                }

                let res = T::try_from(*self.get_apdu_metadata());
                match res {
                    Ok(ins) => {
                        return Some(Event::Command(ins));
                    }
                    Err(_) => {
                        // Invalid Ins code. Send automatically an error, mask
                        // the bad instruction to the application and just
                        // discard this event.
                        self.reply(StatusWords::BadIns);
                    }
                }
            }

            _ => {}
        }
        None
    }

    pub fn reply<T: Into<Reply>>(&mut self, reply: T) {
        let sw = reply.into().0;
        // Append status word
        self.apdu_buffer[self.tx] = (sw >> 8) as u8;
        self.apdu_buffer[self.tx + 1] = sw as u8;
        self.tx += 2;
        // Transmit the response
        self.apdu_send();
    }

    pub fn reply_ok(&mut self) {
        self.reply(StatusWords::Ok);
    }

    pub fn get_apdu_metadata(&self) -> &ApduHeader {
        assert!(self.apdu_buffer.len() >= 4);
        let ptr = &self.apdu_buffer as *const u8 as *const ApduHeader;
        unsafe { &*ptr }
    }

    pub fn get_data(&self) -> Result<&[u8], StatusWords> {
        if self.rx == 4 {
            Ok(&[]) // Conforming zero-data APDU
        } else {
            let first_len_byte = self.apdu_buffer[4] as usize;
            let get_data_from_buffer = |len, offset| {
                if len == 0 || len + offset > self.rx {
                    Err(StatusWords::BadLen)
                } else {
                    Ok(&self.apdu_buffer[offset..offset + len])
                }
            };
            match (first_len_byte, self.rx) {
                (0, 5) => Ok(&[]), // Non-conforming zero-data APDU
                (0, 6) => Err(StatusWords::BadLen),
                (0, _) => {
                    let len =
                        u16::from_le_bytes([self.apdu_buffer[5], self.apdu_buffer[6]]) as usize;
                    get_data_from_buffer(len, 7)
                }
                (len, _) => get_data_from_buffer(len, 5),
            }
        }
    }

    pub fn append(&mut self, m: &[u8]) {
        for c in m.iter() {
            self.apdu_buffer[self.tx] = *c;
            self.tx += 1;
        }
    }

    pub fn append_work_buffer(&mut self, len: usize) {
        for i in 0..len {
            self.apdu_buffer[self.tx] = self.work_buffer[i];
            self.tx += 1;
        }
    }

    pub fn append_work_buffer_from_to(&mut self, start_idx: usize, end_idx: usize) {
        let len = end_idx - start_idx;
        for i in 0..len {
            self.apdu_buffer[self.tx] = self.work_buffer[i + start_idx];
            self.tx += 1;
        }
    }
}

//--------------------------------------------------------------
// Screen Saver/PIN Lock functionality
//--------------------------------------------------------------

fn os_ux_rs(params: &bolos_ux_params_t) {
    unsafe { os_ux(params as *const bolos_ux_params_t as *mut bolos_ux_params_t) };
}
fn last_status() -> u32 {
    unsafe { os_sched_last_status(TASK_BOLOS_UX as u32) as u32 }
}

#[allow(clippy::unnecessary_cast)]
fn scheduler_is_not_running() -> bool {
    let rc = unsafe { os_sched_is_running(TASK_SUBTASKS_START as u32) as u8 };
    rc != BOLOS_TRUE as u8
}

#[repr(u8)]
pub enum UxEvent {
    Event = BOLOS_UX_EVENT,
    WakeUp = BOLOS_UX_WAKE_UP,
}

impl UxEvent {
    pub fn request(&self) -> u32 {
        let params = bolos_ux_params_s {
            ux_id: match self {
                Self::Event => Self::Event as u8,
                Self::WakeUp => Self::WakeUp as u8,
            },
            ..Default::default()
        };

        os_ux_rs(&params);

        last_status()
    }

    pub fn wakeup() {
        if UxEvent::Event.request() == BOLOS_UX_OK {
            UxEvent::WakeUp.request();
        }
    }

    pub fn enter_screen_lock(comm: &mut Comm) -> bool {
        if UxEvent::Event.request() != BOLOS_UX_OK {
            UxEvent::block_and_get_event(comm);
            true
        } else {
            false
        }
    }

    pub fn block_and_get_event(comm: &mut Comm) {
        let mut ret = last_status();

        while ret == BOLOS_UX_IGNORE || ret == BOLOS_UX_CONTINUE {
            if scheduler_is_not_running() {
                let event: Option<Event<Command>> = comm.read_event();

                UxEvent::Event.request();

                if let Some(Event::Command(_)) = event {
                    comm.reply(AppError::NothingReceived);
                }
            } else {
                unsafe { os_sched_yield(BOLOS_UX_OK as u8) };
            }
            ret = last_status();
        }
    }
}
