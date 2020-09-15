use crate::{Status, Handle, LoadedImageProtocol, DevicePathProtocol, system_table};
use crate::boot::EFI_OPEN_PROTOCOL_GET_PROTOCOL;
use core::ptr::null_mut;

// Opaque type for *mut c_void
pub struct ImageHandle;

impl ImageHandle {
    pub fn get_boot_path(&self) -> Result<&'static mut DevicePathProtocol, Status> {
        let self_handle = Handle::from(self);
        let loaded_image = system_table()
            .boot_services
            .open_protocol
            ::<LoadedImageProtocol>(self_handle,
                                    self_handle,
                                    null_mut(),
                                    EFI_OPEN_PROTOCOL_GET_PROTOCOL)?;
        system_table().
            boot_services
            .handle_protocol
            ::<DevicePathProtocol>(loaded_image.device_handle)
    }
}

impl From<&ImageHandle> for Handle {
    fn from(handle: &ImageHandle) -> Self {
        unsafe { &mut *((handle as *const _) as *mut _) }
    }
}

impl From<&mut ImageHandle> for Handle {
    fn from(handle: &mut ImageHandle) -> Self {
        unsafe { &mut *((handle as *mut _) as *mut _) }
    }
}
