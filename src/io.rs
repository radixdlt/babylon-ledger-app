use core::convert::TryFrom;

#[cfg(target_os = "nanox")]
use ledger_device_sdk::ble;
#[cfg(feature = "ccid")]
use ledger_device_sdk::ccid;
use ledger_device_sdk::seph;
pub use ledger_secure_sdk_sys::BOLOS_UX_CANCEL;
pub use ledger_secure_sdk_sys::BOLOS_UX_CONTINUE;
pub use ledger_secure_sdk_sys::BOLOS_UX_ERROR;
pub use ledger_secure_sdk_sys::BOLOS_UX_IGNORE;
pub use ledger_secure_sdk_sys::BOLOS_UX_OK;
pub use ledger_secure_sdk_sys::BOLOS_UX_REDRAW;
use ledger_secure_sdk_sys::buttons::{ButtonEvent, ButtonsState, get_button_event};
use ledger_secure_sdk_sys::seph as sys_seph;

use crate::app_error::AppError;
use crate::command::Command;

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
    pub apdu_buffer: [u8; 260],
    pub rx: usize,
    pub tx: usize,
    buttons: ButtonsState,
    pub work_buffer: [u8; 128],
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
            apdu_buffer: [0u8; 260],
            rx: 0,
            tx: 0,
            buttons: ButtonsState::new(),
            work_buffer: [0u8; 128],
        }
    }

    fn apdu_send(&mut self) {
        if !sys_seph::is_status_sent() {
            sys_seph::send_general_status()
        }
        let mut spi_buffer = [0u8; 128];
        while sys_seph::is_status_sent() {
            sys_seph::seph_recv(&mut spi_buffer, 0);
            seph::handle_event(&mut self.apdu_buffer, &spi_buffer);
        }

        match unsafe { G_io_app.apdu_state } {
            APDU_USB_HID => unsafe {
                io_usb_hid_send(
                    io_usb_send_apdu_data,
                    self.tx as u16,
                    self.apdu_buffer.as_ptr(),
                );
            },
            APDU_RAW => {
                let len = (self.tx as u16).to_be_bytes();
                sys_seph::seph_send(&[sys_seph::SephTags::RawAPDU as u8, len[0], len[1]]);
                sys_seph::seph_send(&self.apdu_buffer[..self.tx]);
            }
            #[cfg(feature = "ccid")]
            APDU_USB_CCID => {
                ccid::send(&self.apdu_buffer[..self.tx]);
            }
            #[cfg(target_os = "nanox")]
            APDU_BLE => {
                ble::send(&self.apdu_buffer[..self.tx]);
            }
            _ => (),
        }
        self.tx = 0;
        self.rx = 0;
        unsafe {
            G_io_app.apdu_state = APDU_IDLE;
            G_io_app.apdu_media = IO_APDU_MEDIA_NONE;
            G_io_app.apdu_length = 0;
        }
    }

    pub fn next_event<T: TryFrom<ApduHeader>>(&mut self) -> Event<T> {
        unsafe {
            G_io_app.apdu_state = APDU_IDLE;
            G_io_app.apdu_media = IO_APDU_MEDIA_NONE;
            G_io_app.apdu_length = 0;
        }

        loop {
            if let Some(value) = self.read_event() {
                return value;
            }
        }
    }

    pub fn read_event<T: TryFrom<ApduHeader>>(&mut self) -> Option<Event<T>> {
        // Signal end of command stream from SE to MCU
        // And prepare reception
        if !sys_seph::is_status_sent() {
            sys_seph::send_general_status();
        }
        // Fetch the next message from the MCU
        let _rx = sys_seph::seph_recv(&mut self.work_buffer, 0);

        // message = [ tag, len_hi, len_lo, ... ]
        let tag = self.work_buffer[0];
        let len = u16::from_be_bytes([self.work_buffer[1], self.work_buffer[2]]);

        match seph::Events::from(tag) {
            seph::Events::ButtonPush => {
                let button_info = self.work_buffer[3] >> 1;
                if let Some(btn_evt) = get_button_event(&mut self.buttons, button_info) {
                    return Some(Event::Button(btn_evt));
                }
            }
            seph::Events::USBEvent => {
                if len == 1 {
                    seph::handle_usb_event(self.work_buffer[3]);
                }
            }
            seph::Events::USBXFEREvent => {
                if len >= 3 {
                    seph::handle_usb_ep_xfer_event(&mut self.apdu_buffer, &self.work_buffer);
                }
            }
            seph::Events::CAPDUEvent => {
                seph::handle_capdu_event(&mut self.apdu_buffer, &self.work_buffer)
            }

            #[cfg(target_os = "nanox")]
            seph::Events::BleReceive => ble::receive(&mut self.apdu_buffer, &self.work_buffer),

            seph::Events::TickerEvent => return Some(Event::Ticker),
            _ => (),
        }

        if unsafe { G_io_app.apdu_state } != APDU_IDLE && unsafe { G_io_app.apdu_length } > 0 {
            self.rx = unsafe { G_io_app.apdu_length as usize };
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
        let ptr = &self.apdu_buffer[0] as &u8 as *const u8 as *const ApduHeader;
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

    pub fn get(&self, start: usize, end: usize) -> &[u8] {
        &self.apdu_buffer[start..end]
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
            if unsafe { os_sched_is_running(TASK_SUBTASKS_START as u32) as u8 } != BOLOS_TRUE as u8
            {
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
