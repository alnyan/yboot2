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
    configuration_table:        *mut ConfigurationTableRaw
}

#[repr(C)]
struct ConfigurationTableRaw {
    vendor_guid:    Guid,
    vendor_table:   *mut c_void
}

pub enum ConfigurationTableEntry {
    Acpi10Table(*mut c_void),       // RSDP
    NotImplemented(*mut c_void)
}

pub struct ConfigurationTableIterator<'a> {
    index:  usize,
    st:     &'a SystemTable
}

const EFI_ACPI_10_TABLE_GUID: Guid = Guid {
    data1: 0xeb9d2d30,
    data2: 0x2d88,
    data3: 0x11d3,
    data4: [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]
};
impl From<&ConfigurationTableRaw> for ConfigurationTableEntry {
    fn from(raw: &ConfigurationTableRaw) -> Self {
        use ConfigurationTableEntry::*;
        match &raw.vendor_guid {
            &EFI_ACPI_10_TABLE_GUID => Acpi10Table(raw.vendor_table),
            _                       => NotImplemented(raw.vendor_table)
        }
    }
}

impl<'a> Iterator for ConfigurationTableIterator<'a> {
    type Item = ConfigurationTableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.st.number_of_table_entries {
            None
        } else {
            let raw = unsafe {&*self.st.configuration_table.offset(self.index as isize)};
            self.index += 1;
            Some(ConfigurationTableEntry::from(raw))
        }
    }
}

impl SystemTable {
    pub fn config_iter(&self) -> ConfigurationTableIterator {
        ConfigurationTableIterator {
            st: self,
            index: 0
        }
    }
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
