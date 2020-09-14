#![feature(never_type)]
#![no_std]

use core::ffi::c_void;
use core::ptr::null_mut;

pub mod proto;
pub use proto::*;

pub mod base;
pub use base::*;

pub mod mmap;
pub use mmap::*;

pub mod boot;
pub use boot::BootServices;
pub mod runtime;
pub use runtime::RuntimeServices;

#[repr(C)]
pub struct TableHeader {
    signature:      u64,
    revision:       u32,
    header_size:    u32,
    crc32:          u32,
    reserved:       u32
}

#[repr(C)]
pub struct SystemTable {
    hdr:                        TableHeader,
    pub firmware_vendor:        *const i16,
    pub firmware_revision:      u32,
    console_in_handle:          Handle,
    pub con_in:                 &'static mut SimpleTextInputProtocol,
    console_out_handle:         Handle,
    pub con_out:                &'static mut SimpleTextOutputProtocol,
    standard_error_handle:      Handle,
    pub std_err:                &'static mut SimpleTextOutputProtocol,
    pub runtime_services:       &'static mut RuntimeServices,
    pub boot_services:          &'static mut BootServices,
    number_of_table_entries:    usize,
    configuration_table:        *mut c_void         // TODO
}

static mut SYSTEM_TABLE: *mut SystemTable = null_mut();
static mut IMAGE_HANDLE: Handle = null_mut();

pub fn init(ih: Handle, st: *mut SystemTable) {
    if st.is_null() || ih.is_null() {
        panic!();
    }
    unsafe {
        SYSTEM_TABLE = st;
        IMAGE_HANDLE = ih;
    }
}

pub fn system_table() -> &'static mut SystemTable {
    unsafe { &mut *SYSTEM_TABLE }
}

pub fn image_handle() -> Handle {
    unsafe { &mut *IMAGE_HANDLE }
}
