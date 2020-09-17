use crate::{CStr16, Status};
use core::ptr::null_mut;
use core::ffi::c_void;

pub const OPEN_MODE_READ: u64 = 1;

#[repr(C)]
pub struct FileProtocol {
    revision:       u64,
    open:           unsafe fn (*const FileProtocol,
                               *mut *mut FileProtocol,
                               *const u16,
                               u64, u64) -> u64,
    close:          unsafe fn (*mut FileProtocol) -> u64,
    delete:         *mut c_void,
    read:           unsafe fn (*mut FileProtocol, *mut usize, *mut c_void) -> u64,
    write:          *mut c_void,
    get_position:   *mut c_void,
    set_position:   unsafe fn (*mut FileProtocol, u64) -> u64,
}

pub struct File {
    inner: *mut FileProtocol
}

impl From<*mut FileProtocol> for File {
    fn from(proto: *mut FileProtocol) -> File {
        File {
            inner: proto
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            ((*self.inner).close)(
                self.inner
            );
        }
        self.inner = null_mut();
    }
}

impl File {
    pub fn open(&self, name: &CStr16, mode: u64, attr: u64) -> Result<File, Status> {
        let mut ptr: *mut FileProtocol = null_mut();
        match Status::from(unsafe {
            ((*self.inner).open)(
                self.inner,
                &mut ptr,
                name.as_ptr(),
                mode,
                attr
            )
        }) {
            Status::Success => Ok(File::from(ptr)),
            err             => Err(err)
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, Status> {
        let mut len = buf.len();
        match Status::from(unsafe {
            ((*self.inner).read)(
                self.inner,
                &mut len,
                buf as *mut _ as *mut _
            )
        }) {
            Status::Success => Ok(len),
            err             => Err(err)
        }
    }

    pub fn seek(&mut self, pos: u64) -> Result<(), Status> {
        Status::from(unsafe {
            ((*self.inner).set_position)(
                self.inner,
                pos
            )
        }).into()
    }
}
