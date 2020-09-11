use core::ffi::c_void;
use super::{
    Event,
    Guid,
    Handle,
    TableHeader,
    Status,
    MemoryMap,
    MemoryDescriptor
};
use crate::proto::Protocol;

#[repr(C)]
pub struct BootServices {
    hdr:                            TableHeader,
    raise_tpl:                      *mut c_void,
    restore_tpl:                    *mut c_void,
    allocate_pages:                 *mut c_void,
    free_pages:                     *mut c_void,
    get_memory_map:                 unsafe fn (*mut usize,
                                               *mut MemoryDescriptor,
                                               *mut usize,
                                               *mut usize,
                                               *mut u32) -> u64,
    allocate_pool:                  *mut c_void,
    free_pool:                      *mut c_void,
    create_event:                   *mut c_void,
    set_timer:                      *mut c_void,
    wait_for_event:                 unsafe fn (usize, *const Event, *mut usize) -> u64,
    signal_event:                   *mut c_void,
    close_event:                    *mut c_void,
    check_event:                    *mut c_void,
    install_protocol_interface:     *mut c_void,
    reinstall_protocol_interface:   *mut c_void,
    uninstall_protocol_interface:   *mut c_void,
    handle_protocol:                *mut c_void,
    reserved:                       *mut c_void,
    register_protocol_notify:       *mut c_void,
    locate_handle:                  *mut c_void,
    locate_device_path:             *mut c_void,
    install_configuration_table:    *mut c_void,
    image_load:                     *mut c_void,
    image_start:                    *mut c_void,
    exit:                           unsafe fn (Handle, u64, usize, *const i16) -> !,
    image_unload:                   *mut c_void,
    exit_boot_services:             unsafe fn (Handle, usize) -> u64,
    get_next_monotonic_count:       *mut c_void,
    stall:                          unsafe fn (u64) -> (),
    set_watchdog_timer:             *mut c_void,
    connect_controller:             *mut c_void,
    disconnect_controller:          *mut c_void,
    open_protocol:                  *mut c_void,
    close_protocol:                 *mut c_void,
    open_protocol_information:      *mut c_void,
    protocols_per_handle:           *mut c_void,
    locate_handle_buffer:           *mut c_void,
    locate_protocol:                unsafe fn (*const Guid, *mut c_void, *mut *mut c_void) -> u64,
    /*
    EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES    InstallMultipleProtocolInterfaces;
    EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES  UninstallMultipleProtocolInterfaces;
    EFI_CALCULATE_CRC32                         CalculateCrc32;
    EFI_COPY_MEM                                CopyMem;
    EFI_SET_MEM                                 SetMem;
    EFI_CREATE_EVENT_EX                         CreateEventEx;
*/
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

    // Unlike EFI's variant, just for one event
    pub fn wait_for_event(&self, ev: Event) -> Status {
        let mut index = 0usize;
        return Status::from_num(unsafe {
            (self.wait_for_event)(
                1,
                &ev,
                &mut index
            )
        });
    }

    pub fn exit(&self, status: Status) -> ! {
        unsafe {
            (self.exit)(super::image_handle(), status.to_num(), 0, core::ptr::null());
        }
    }

    pub fn stall(&self, micros: u64) {
        unsafe { (self.stall)(micros) }
    }

    pub fn exit_boot_services(&self, map_key: usize) -> Status {
        return Status::from_num(unsafe {
            (self.exit_boot_services)(super::image_handle(), map_key)
        });
    }

    pub fn locate_protocol<T: Protocol>(&self) -> Result<&'static mut T, Status> {
        let guid = <T as Protocol>::guid();
        let mut proto_ptr: *mut c_void = core::ptr::null_mut();

        match Status::from_num(unsafe {
            (self.locate_protocol)(
                guid as *const Guid,
                core::ptr::null_mut(),
                (&mut proto_ptr) as *mut *mut c_void
            )
        }) {
            Status::Success => Ok(unsafe {(proto_ptr as *mut T).as_mut()}.unwrap()),
            err             => Err(err)
        }
    }
}
