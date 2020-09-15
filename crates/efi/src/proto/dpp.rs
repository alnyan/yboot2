use crate::{Protocol, Guid, Status, Handle, SimpleFileSystemProtocol, File, system_table};
use core::ffi::c_void;
use core::ptr::null_mut;
use core::fmt;

const EFI_END_OF_HARDWARE_DEVICE_PATH: u8 = 0x7F;

#[repr(C)]
pub struct DevicePathProtocol {
    _type:      u8,
    subtype:    u8,
    length:     [u8; 2]
}

impl Protocol for DevicePathProtocol {
    const GUID: Guid = Guid {
        data1: 0x09576e91,
        data2: 0x6d3f,
        data3: 0x11d2,
        data4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]
    };
}

impl PartialEq for DevicePathProtocol {
    fn eq(&self, other: &DevicePathProtocol) -> bool {
        extern "C" {
            fn memcmp(a: *const c_void, b: *const c_void, len: usize) -> isize;
        }

        let mut dp1 = self;
        let mut dp2 = other;

        loop {
            if dp1._type != dp2._type {
                return false;
            }

            if dp1.subtype != dp2.subtype {
                return false;
            }

            let len1 = dp1.node_length();
            let len2 = dp2.node_length();
            if len1 != len2 {
                return false;
            }

            if unsafe {memcmp(dp1 as *const _ as *const _,
                              dp2 as *const _ as *const _,
                              len1 as usize)} != 0 {
                return false;
            }

            if dp1.is_end() {
                return true;
            }

            dp1 = unsafe { &*((dp1 as *const _ as *const u8).offset(len1 as isize) as *const _) };
            dp2 = unsafe { &*((dp2 as *const _ as *const u8).offset(len2 as isize) as *const _) };
        }
    }
}

impl DevicePathProtocol {
    pub fn open_partition(&self) -> Result<File, Status> {
        use crate::boot::LocateSearchType::ByProtocol;

        let mut iter = system_table()
            .boot_services
            .handle_buffer_iter
            ::<SimpleFileSystemProtocol>(ByProtocol, null_mut())?;

        for item in iter {
            let path = system_table()
                .boot_services
                .handle_protocol
                ::<DevicePathProtocol>(item)?;

            if path == self {
                let fs = system_table()
                    .boot_services
                    .handle_protocol
                    ::<SimpleFileSystemProtocol>(item)?;
                return fs.open_volume();
            }
        }

        // TODO not found?
        Err(Status::InvalidParameter)
    }

    pub fn node_length(&self) -> u16 {
        return ((self.length[1] as u16) << 8) | self.length[0] as u16;
    }

    pub fn is_end(&self) -> bool {
        ((self._type == EFI_END_OF_HARDWARE_DEVICE_PATH) &&
         (self.subtype == 0xFF))
    }
}
