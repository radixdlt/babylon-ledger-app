use core::mem::MaybeUninit;
use core::slice::from_raw_parts;
use core::stringify;

use paste::paste;

use crate::print::parameter_printer::ParameterPrinter;
use crate::print::state::ParameterPrinterState;
use crate::print::state::TITLE_SIZE;
use crate::sbor_decoder::SborEvent;
use crate::static_vec::StaticVec;

// Parameter which we just skip, without printing anything
pub struct IgnoredParameter {}

pub const IGNORED_PARAMETER_PRINTER: IgnoredParameter = IgnoredParameter {};

impl<T> ParameterPrinter<T> for IgnoredParameter {
    fn handle_data(&self, _state: &mut ParameterPrinterState<T>, _event: SborEvent) {}

    fn end(&self, state: &mut ParameterPrinterState<T>) {
        state.print_text(b"<UNKNOWN TYPE>")
    }
}

// BOOL parameter printer
pub struct BoolParameterPrinter {}

pub const BOOL_PARAMETER_PRINTER: BoolParameterPrinter = BoolParameterPrinter {};

impl<T> ParameterPrinter<T> for BoolParameterPrinter {
    fn end(&self, state: &mut ParameterPrinterState<T>) {
        if state.data.len() != 1 {
            state.print_text(b"<Invalid bool encoding>");
            return;
        }

        let message: &[u8] = match state.data.as_slice()[0] {
            0 => b"false",
            1 => b"true",
            _ => b"(invalid bool)",
        };

        state.print_text(message);
    }
}

// String parameter printer
pub struct StringParameterPrinter {}

pub const STRING_PARAMETER_PRINTER: StringParameterPrinter = StringParameterPrinter {};

impl<T> ParameterPrinter<T> for StringParameterPrinter {
    fn end(&self, state: &mut ParameterPrinterState<T>) {
        state.print_byte(b'"');
        state.print_data_as_text();
        state.print_byte(b'"');
    }
}

// Helper macros/functions are derived from https://github.com/japaric/ufmt
// Original copyright (MIT License):
// Copyright (c) 2019 Jorge Aparicio
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

macro_rules! ixx {
    ($uxx:ty, $n:expr, $buf:expr) => {{
        let ptr = $buf.as_mut_ptr().cast::<u8>();
        let len = $buf.len();
        let n = $n;
        let negative = n.is_negative();
        let mut n = if negative {
            match n.checked_abs() {
                Some(n) => n as $uxx,
                None => <$uxx>::max_value() / 2 + 1,
            }
        } else {
            n as $uxx
        };
        let mut i = len - 1;
        loop {
            unsafe { ptr.add(i).write((n % 10) as u8 + b'0') }
            n /= 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        if negative {
            i -= 1;
            unsafe { ptr.add(i).write(b'-') }
        }

        unsafe { from_raw_parts(ptr.add(i), len - i) }
    }};
}

macro_rules! uxx {
    ($n:expr, $buf:expr) => {{
        let ptr = $buf.as_mut_ptr().cast::<u8>();
        let len = $buf.len();
        let mut n = $n;
        let mut i = len - 1;
        loop {
            unsafe { ptr.add(i).write((n % 10) as u8 + b'0') }
            n /= 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        unsafe { from_raw_parts(ptr.add(i), len - i) }
    }};
}
// End of derived code

macro_rules! printer_for_utype {
    ($type:ty) => {
        paste! {
            pub struct [<$type:upper ParameterPrinter>] {}
            pub const [<$type:upper _PARAMETER_PRINTER>] : [<$type:upper ParameterPrinter>] = [<$type:upper ParameterPrinter>] {};

            impl [<$type:upper ParameterPrinter>] {
                pub fn print<T>(state: &mut ParameterPrinterState<T>, number: $type) {
                    let mut buf = [MaybeUninit::<u8>::uninit(); 40];
                    let bytes = uxx!(number, buf);

                    state.print_text(bytes);
                    state.print_text(stringify!($type).as_bytes());
                }
            }

            impl<T> ParameterPrinter<T> for [<$type:upper ParameterPrinter>] {
                fn handle_data(
                    &self,
                    state: &mut ParameterPrinterState<T>,
                    event: SborEvent
                ) {
                    if let SborEvent::Data(byte) = event {
                        state.push_byte(byte);
                    }
                }

                fn end(&self, state: &mut ParameterPrinterState<T>) {
                    if state.data.len() != (($type::BITS / 8) as usize) {
                        state.print_text(b"<Invalid encoding>");
                        return;
                    }
                    fn to_array(input: &[u8]) -> [u8; ($type::BITS / 8) as usize] {
                        input.try_into().expect("<should not happen>")
                    }

                    [<$type:upper ParameterPrinter>]::print(state, $type::from_le_bytes(to_array(state.data.as_slice())));
                }
            }
        }
    };
}

macro_rules! printer_for_itype {
    ($type:ty, $utype:ty) => {
        paste! {
            pub struct [<$type:upper ParameterPrinter>] {}
            pub const [<$type:upper _PARAMETER_PRINTER>] : [<$type:upper ParameterPrinter>] = [<$type:upper ParameterPrinter>] {};

            impl [<$type:upper ParameterPrinter>] {
                pub fn print<T>(state: &mut ParameterPrinterState<T>, number: $type) {
                    let mut buf = [MaybeUninit::<u8>::uninit(); 40];
                    let bytes = ixx!($utype, number, buf);

                    state.print_text(bytes);
                    state.print_text(stringify!($type).as_bytes());
                }
            }

            impl<T> ParameterPrinter<T> for [<$type:upper ParameterPrinter>] {
                fn handle_data(
                    &self,
                    state: &mut ParameterPrinterState<T>,
                    event: SborEvent
                ) {
                    if let SborEvent::Data(byte) = event {
                        state.push_byte(byte);
                    }
                }

                fn end(&self, state: &mut ParameterPrinterState<T>) {
                    if state.data.len() != (($type::BITS / 8) as usize) {
                        state.print_text(b"<Invalid encoding>");
                        return;
                    }
                    fn to_array(input: &[u8]) -> [u8; ($type::BITS / 8) as usize] {
                        input.try_into().expect("<should not happen>")
                    }

                    [<$type:upper ParameterPrinter>]::print(state, $type::from_le_bytes(to_array(state.data.as_slice())));
                }
            }
        }
    };
}

printer_for_utype!(u8);
printer_for_utype!(u16);
printer_for_utype!(u32);
printer_for_utype!(u64);
printer_for_utype!(u128);
printer_for_itype!(i8, u8);
printer_for_itype!(i16, u16);
printer_for_itype!(i32, u32);
printer_for_itype!(i64, u64);
printer_for_itype!(i128, u128);

// Standalone printer for u32
pub fn print_u32(output: &mut StaticVec<u8, { TITLE_SIZE }>, number: u32) {
    let mut buf = [MaybeUninit::<u8>::uninit(); 12];
    let bytes = uxx!(number, buf);

    output.extend_from_slice(bytes);
}
