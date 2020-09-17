use crate::{CStr16, Status, Guid};
use core::ptr::null_mut;
use core::ffi::c_void;

pub const OPEN_MODE_READ: u64 = 1;
pub const FILE_INFO_GUID: Guid = Guid {
    data1: 0x09576e92,
    data2: 0x6d3f,
    data3: 0x11d2,
    data4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]
};

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
    get_info:       unsafe fn (*mut FileProtocol,
                               *const Guid,
                               *mut usize,
                               *mut c_void) -> u64,
}

#[repr(C)]
pub struct Stat {
    pub size:       u64,
    pub file_size:  u64,
    // ...
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

    pub fn stat(&mut self, statbuf: &mut [u8]) -> Result<&Stat, Status> {
        let mut len = statbuf.len();
        match Status::from(unsafe {
            ((*self.inner).get_info)(
                self.inner,
                &FILE_INFO_GUID,
                &mut len,
                statbuf.as_mut_ptr() as *mut _
            )
        }) {
            Status::Success     => Ok(unsafe {core::mem::transmute(statbuf.as_ptr())}),
            err                 => Err(err)
        }
    }
}
