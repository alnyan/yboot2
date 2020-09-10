#![no_std]

use core::ffi::c_void;
use core::ptr::null_mut;

pub mod proto;
pub use proto::stop::*;

pub mod base;
pub use base::*;

pub mod mmap;
pub use mmap::*;

pub mod boot;
pub use boot::BootServices;

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
    con_in:                     *mut c_void,    // TODO
    console_out_handle:         Handle,
    pub con_out:                &'static mut SimpleTextOutputProtocol,
    standard_error_handle:      Handle,
    pub std_err:                &'static mut SimpleTextOutputProtocol,
    runtime_services:           *mut c_void,        // TODO
    pub boot_services:          &'static mut BootServices,  // TODO
    number_of_table_entries:    usize,
    configuration_table:        *mut c_void         // TODO
}

static mut SYSTEM_TABLE: *mut SystemTable = null_mut();

pub fn init_tables(st: *mut SystemTable) {
    if st.is_null() {
        panic!();
    }
    unsafe { SYSTEM_TABLE = st; }
}

pub fn system_table() -> &'static mut SystemTable {
    unsafe { &mut *SYSTEM_TABLE }
}
