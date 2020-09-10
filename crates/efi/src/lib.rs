#![no_std]

use core::ffi::c_void;
use core::ptr::null_mut;

pub mod proto;
pub use proto::stop::*;

pub mod base;
pub use base::*;

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
    con_out:                    *mut SimpleTextOutputProtocol,
    standard_error_handle:      Handle,
    std_err:                    *mut SimpleTextOutputProtocol,
    runtime_services:           *mut c_void,    // TODO
    boot_services:              *mut c_void,    // TODO
    number_of_table_entries:    usize,
    configuration_table:        *mut c_void     // TODO
}

impl SystemTable {
    pub fn con_out(&self) -> &'static mut SimpleTextOutputProtocol {
        unsafe { &mut *self.con_out }
    }
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
