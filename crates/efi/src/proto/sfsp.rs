use crate::{Protocol, Guid, Status, File, FileProtocol};
use core::ptr::null_mut;

#[repr(C)]
pub struct SimpleFileSystemProtocol {
    revision:       u64,
    open_volume:    unsafe fn (*mut SimpleFileSystemProtocol, *mut *mut FileProtocol) -> u64
}

impl Protocol for SimpleFileSystemProtocol {
    const GUID: Guid = Guid {
        data1: 0x0964e5b22,
        data2: 0x6459,
        data3: 0x11d2,
        data4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]
    };
}

impl SimpleFileSystemProtocol {
    pub fn open_volume(&mut self) -> Result<File, Status> {
        let mut root: *mut FileProtocol = null_mut();

        match Status::from(unsafe {
            (self.open_volume)(
                self,
                &mut root
            )
        }) {
            Status::Success => Ok(File::from(root)),
            err             => Err(err)
        }
    }
}
