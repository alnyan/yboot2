use crate::{Protocol, Guid};
use core::ffi::c_void;

#[repr(C)]
pub struct FileProtocol {
    revision:   u64,
    open:       *mut c_void,
    close:      unsafe fn (*mut FileProtocol) -> u64,
    delete:     *mut c_void,
    read:       *mut c_void,
    write:      *mut c_void,
    // ...
}

pub struct File<'a> {
    inner: &'a mut FileProtocol
}

impl<'a> From<*mut FileProtocol> for File<'a> {
    fn from(proto: *mut FileProtocol) -> File<'a> {
        File {
            inner: unsafe { &mut *proto }
        }
    }
}

impl<'a> Drop for File<'a> {
    fn drop(&mut self) {
        unsafe {
            (self.inner.close)(
                self.inner
            );
        }
    }
}
