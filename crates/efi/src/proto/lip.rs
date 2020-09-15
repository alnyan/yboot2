use crate::{
    Protocol,
    Guid,
    SystemTable,
    DevicePathProtocol,
    Handle
};
use core::ffi::c_void;

#[repr(C)]
pub struct LoadedImageProtocol {
    pub revision:           u32,
    pub parent_handle:      Handle,
    system_table:           &'static mut SystemTable,
    pub device_handle:      Handle,
    pub file_path:          &'static mut DevicePathProtocol,
    reserved:               *mut c_void,
    load_options_size:      u32,
    load_options:           *mut c_void,
    pub image_base:         *mut c_void,
    pub image_size:         u64,
    pub image_code_type:    usize,
    unload:                 *mut c_void     // TODO
}

impl Protocol for LoadedImageProtocol {
    const GUID: Guid = Guid {
        data1: 0x5b1b31a1,
        data2: 0x9562,
        data3: 0x11d2,
        data4: [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]
    };
}
