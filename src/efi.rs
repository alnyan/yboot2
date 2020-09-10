use core::ffi::c_void;
use spin::Mutex;

pub enum Status {
    Success,
    Err
}

impl Status {
    pub fn to_isize(self) -> isize {
        match self {
            Status::Success =>      0,
            Status::Err =>          -1,
        }
    }
    pub fn from_isize(v: isize) -> Status {
        match v {
            0   => Status::Success,
            _   => Status::Err
        }
    }
}

#[repr(C)]
pub struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    fn_reset: *mut c_void,
    fn_output_string: unsafe fn(&SimpleTextOutputProtocol, s: *const i16) -> isize
}

impl SimpleTextOutputProtocol {
    pub fn output_string(&self, s: *const i16) -> Status {
        unsafe { Status::from_isize((self.fn_output_string)(self, s)) }
    }
}

#[repr(C)]
pub struct SystemTable {
    hdr:                        TableHeader,
    firmware_vendor:            *const i16,
    firmware_revision:          u32,
    console_in_handle:          *mut c_void,
    con_in:                     *mut c_void,
    console_out_handle:         *mut c_void,
    con_out:                    *mut SimpleTextOutputProtocol,
    standard_error_handle:      *mut c_void,
    std_err:                    *mut c_void,
    runtime_services:           *mut c_void,
    boot_services:              *mut c_void,
    number_of_table_entries:    usize,
    configuration_table:        *mut c_void
}

impl SystemTable {
    pub fn con_out(&self) -> &'static mut SimpleTextOutputProtocol {
        unsafe { &mut *self.con_out }
    }
}

static mut SYSTEM_TABLE: Option<*mut SystemTable> = None;

pub fn init_tables(st: *mut SystemTable) {
    unsafe { SYSTEM_TABLE = Some(st); }
}

pub fn system_table() -> &'static mut SystemTable {
    unsafe { &mut *SYSTEM_TABLE.unwrap() }
}

pub fn con_output_string(text: &str) {
    let mut buf = [0i16; 512];
    let mut i = 0;

    for byte in text.bytes() {
        buf[i] = byte as i16;
        i += 1;
    }

    system_table().con_out().output_string(buf.as_ptr());
}
