use core::str::from_utf8;
use core::arch::asm;

#[cfg(test)]
pub fn debug_print(_s: &str) {}
#[cfg(test)]
pub fn debug_prepared_messafe(_s: &[u8]) {}
#[cfg(test)]
pub fn debug_print_byte(_s: u8) {}

#[cfg(not(test))]
/// Debug 'print' function that uses ARM semihosting
/// Prints only strings with no formatting
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

#[cfg(not(test))]
pub fn debug_prepared_message(message: &[u8]) {
    debug_print(from_utf8(message).unwrap());
    debug_print("\n");
}

#[cfg(not(test))]
pub fn debug_print_byte(byte: u8) {
    let mut buffer = [0u8; 1];
    buffer[0] = byte;
    debug_print(from_utf8(&buffer).unwrap());
}