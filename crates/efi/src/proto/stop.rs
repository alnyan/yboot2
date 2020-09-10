use crate::Status;

use core::ffi::c_void;
use core::fmt;

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    fn_reset: *mut c_void,
    fn_output_string: unsafe fn(&SimpleTextOutputProtocol, s: *const i16) -> isize
}

impl SimpleTextOutputProtocol {
    // Not the original name provided by EFI, but whatever
    pub fn output_char16_string(&self, s: *const i16) -> Status {
        unsafe { Status::from_isize((self.fn_output_string)(self, s)) }
    }

    pub fn output_string(&self, s: &str) {
        let mut buf = [0i16; 64];
        let mut iter = s.bytes();
        let mut i = 0;

        while let Some(byte) = iter.next() {
            if i == 63 {
                buf[i] = 0;
                i = 0;

                // Output chunk
                self.output_char16_string(buf.as_ptr());
            } else {
                buf[i] = byte as i16;
                i += 1;
            }
        }

        if i != 0 {
            buf[i] = 0;
            self.output_char16_string(buf.as_ptr());
        }
    }
}

impl fmt::Write for SimpleTextOutputProtocol {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.output_string(s);
        Ok(())
    }
}
