use core::arch::asm;
use core::str::from_utf8;

#[cfg(test)]
pub fn debug_print(_s: &str) {}
#[cfg(test)]
pub fn debug_prepared_message(_s: &[u8]) {}
#[cfg(test)]
pub fn debug_print_byte(_s: u8) {}
#[cfg(test)]
pub fn debug_print_hex_byte(_s: u8) {}

/// Debug 'print' function that uses ARM semihosting
/// Prints only strings with no formatting
#[cfg(all(
    not(test),
    any(target_os = "nanos", target_os = "nanox", target_os = "nanosplus")
))]
pub fn debug_print(s: &str) {
    let p = s.as_bytes().as_ptr();
    for i in 0..s.len() {
        let m = unsafe { p.add(i) };
        unsafe {
            asm!(
            "svc #0xab",
            in("r1") m,
            inout("r0") 3 => _,
            );
        }
    }
}
#[cfg(all(
    not(test),
    not(any(target_os = "nanos", target_os = "nanox", target_os = "nanosplus"))
))]
pub fn debug_print(_s: &str) {}

#[cfg(not(test))]
pub fn debug_prepared_message(message: &[u8]) {
    debug_print(from_utf8(message).unwrap());
    debug_print("\n");
}

#[cfg(not(test))]
pub fn debug_print_byte(byte: u8) {
    let mut buffer = [0u8; 3];
    buffer[0] = b'<';
    buffer[1] = byte;
    buffer[2] = b'>';
    debug_prepared_message(&buffer);
}

#[cfg(all(
    not(test),
    any(target_os = "nanos", target_os = "nanox", target_os = "nanosplus")
))]
const HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";

#[cfg(all(
    not(test),
    any(target_os = "nanos", target_os = "nanox", target_os = "nanosplus")
))]
pub fn debug_print_hex_byte(byte: u8) {
    let mut buffer = [0u8; 2];
    buffer[0] = HEX_DIGITS[((byte >> 4) & 0x0F) as usize];
    buffer[1] = HEX_DIGITS[(byte & 0x0F) as usize];
    debug_prepared_message(&buffer);
}
#[cfg(all(
    not(test),
    not(any(target_os = "nanos", target_os = "nanox", target_os = "nanosplus"))
))]
pub fn debug_print_hex_byte(_s: &str) {}
