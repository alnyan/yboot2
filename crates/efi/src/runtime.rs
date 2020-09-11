use crate::{TableHeader, Status};
use core::ffi::c_void;

#[repr(C)]
pub struct RuntimeServices {
    hdr:        TableHeader,
    get_time:   unsafe fn (*mut u64, *mut c_void) -> u64
}

impl RuntimeServices {
    pub fn get_time(&self) -> Result<u64, Status> {
        let mut time = 0u64;
        return match Status::from_num(unsafe {
            (self.get_time)((&mut time) as *mut u64, core::ptr::null_mut())
        }) {
            Status::Success => Ok(time),
            err             => Err(err)
        };
    }
}
