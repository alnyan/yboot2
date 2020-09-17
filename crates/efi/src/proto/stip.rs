use crate::{Status, Guid, Protocol, Event, system_table};

#[repr(C)]
#[derive(Debug)]
pub struct InputKey {
    pub scan_code:      u16,
    pub unicode_char:   u16
}

#[repr(C)]
pub struct SimpleTextInputProtocol {
    reset:              unsafe fn (*mut SimpleTextInputProtocol, bool) -> u64,
    read_key_stroke:    unsafe fn (*mut SimpleTextInputProtocol, *mut InputKey) -> u64,
    wait_for_key:       Event
}

impl Protocol for SimpleTextInputProtocol {
    const GUID: Guid = Guid {
        data1: 0x387477c1,
        data2: 0x69c7,
        data3: 0x11d2,
        data4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]
    };
}

impl SimpleTextInputProtocol {
    pub fn reset(&mut self, extended_verification: bool) -> Result<(), Status> {
        Status::from(unsafe {
            (self.reset)(self as *mut SimpleTextInputProtocol, extended_verification)
        }).into()
    }

    pub fn read_key_blocking(&mut self) -> Result<InputKey, Status> {
        system_table().boot_services.wait_for_event(self.wait_for_key)?;
        loop {
            let res = self.read_key_stroke();
            match res {
                Err(Status::NotReady)   => continue,
                Ok(key)                 => return Ok(key),
                err                     => return err
            }
        }
    }

    pub fn read_key_stroke(&mut self) -> Result<InputKey, Status> {
        let mut stroke = InputKey { scan_code: 0, unicode_char: 0 };
        match Status::from(unsafe {
            (self.read_key_stroke)(
                self as *mut SimpleTextInputProtocol,
                (&mut stroke) as *mut InputKey
            )
        }) {
            Status::Success => Ok(stroke),
            err             => Err(err)
        }
    }
}
