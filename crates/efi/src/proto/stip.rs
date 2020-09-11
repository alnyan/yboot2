use crate::{Status, Guid, Event, system_table};
use core::ffi::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct InputKey {
    pub scan_code:      u16,
    pub unicode_char:   u16
}

#[repr(C)]
pub struct SimpleTextInputProtocol {
    reset:              unsafe fn (*mut SimpleTextInputProtocol, bool) -> u64,
    read_key_stroke:    unsafe fn (*mut SimpleTextInputProtocol, *mut InputKey) -> u64,
    wait_for_key:       Event
}

impl SimpleTextInputProtocol {
    pub fn reset(&mut self, extended_verification: bool) -> Status {
        return Status::from_num(unsafe {
            (self.reset)(self as *mut SimpleTextInputProtocol, extended_verification)
        });
    }

    pub fn read_key_blocking(&mut self) -> Result<InputKey, Status> {
        let res = system_table().boot_services.wait_for_event(self.wait_for_key);
        if res != Status::Success {
            return Err(res);
        }
        loop {
            let res = self.read_key_stroke();
            match res {
                Err(Status::NotReady)   => continue,
                Ok(key)                 => return Ok(key),
                err                     => return err
            }
        }
    }

    pub fn read_key_stroke(&mut self) -> Result<InputKey, Status> {
        let mut stroke = InputKey { scan_code: 0, unicode_char: 0 };
        return match Status::from_num(unsafe {
            (self.read_key_stroke)(
                self as *mut SimpleTextInputProtocol,
                (&mut stroke) as *mut InputKey
            )
        }) {
            Status::Success => Ok(stroke),
            err             => Err(err)
        };
    }
}
