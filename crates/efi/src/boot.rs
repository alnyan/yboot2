use core::ffi::c_void;
use super::{TableHeader, Status, MemoryMap, MemoryDescriptor};

#[repr(C)]
pub struct BootServices {
    hdr:            TableHeader,
    raise_tpl:      *mut c_void,
    restore_tpl:    *mut c_void,
    allocate_pages: *mut c_void,
    free_pages:     *mut c_void,
    get_memory_map: unsafe fn (*mut usize,
                               *mut MemoryDescriptor,
                               *mut usize,
                               *mut usize,
                               *mut u32) -> u64
}

impl BootServices {
    pub fn get_memory_map(&self, out: &mut MemoryMap) -> Status {
        out.size = out.storage_ref.len();
        return Status::from_num(unsafe {
            let mut trash: u32 = 0;
            (self.get_memory_map)(
                (&mut out.size)                 as *mut usize,
                out.storage_ref.as_mut_ptr()    as *mut MemoryDescriptor,
                (&mut out.key)                  as *mut usize,
                (&mut out.descriptor_size)      as *mut usize,
                (&mut trash)                    as *mut u32
            )
        });
    }
}
