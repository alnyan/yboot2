use core::ffi::c_void;
use core::fmt;

pub type Handle = *mut c_void;
pub type Event = *mut c_void;

pub type Result<T> = core::result::Result<T, Status>;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Status {
    Success,
    Err,
    InvalidParameter,
    BufferTooSmall,
    NotReady
}

pub trait Termination {
    fn to_efi(&self) -> u64;
}

const EFI_SUCCESS:                  u64 = 0;
const EFI_ERR:                      u64 = 0x8000000000000000;
const EFI_INVALID_PARAMETER:        u64 = EFI_ERR | 0x02;
const EFI_BUFFER_TOO_SMALL:         u64 = EFI_ERR | 0x05;
const EFI_NOT_READY:                u64 = EFI_ERR | 0x06;

impl Status {
    pub fn to_num(self) -> u64 {
        match self {
            Status::Success             => EFI_SUCCESS,
            Status::Err                 => EFI_ERR,
            Status::InvalidParameter    => EFI_INVALID_PARAMETER,
            Status::BufferTooSmall      => EFI_BUFFER_TOO_SMALL,
            Status::NotReady            => EFI_NOT_READY,
        }
    }
    pub fn from_num(v: u64) -> Status {
        match v {
            0                       => Status::Success,
            EFI_ERR                 => Status::Err,
            EFI_INVALID_PARAMETER   => Status::InvalidParameter,
            EFI_BUFFER_TOO_SMALL    => Status::BufferTooSmall,
            EFI_NOT_READY           => Status::NotReady,
            _                       => Status::Err
        }
    }

    pub fn to_result(&self) -> Result<()> {
        match self {
            Status::Success => Ok(()),
            err             => Err(*err)
        }
    }
}

impl Termination for () {
    fn to_efi(&self) -> u64 {
        return EFI_SUCCESS;
    }
}

impl Termination for ! {
    fn to_efi(&self) -> u64 {
        return EFI_ERR;
    }
}

impl<T> Termination for Result<T> {
    fn to_efi(&self) -> u64 {
        match self {
            Ok(_)       => EFI_SUCCESS,
            Err(err)    => {
                assert!(*err != Status::Success);
                err.to_num()
            }
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct Guid {
    pub data1:  u32,
    pub data2:  u16,
    pub data3:  u16,
    pub data4:  [u8; 8]
}

#[repr(transparent)]
pub struct CStr16 {
    data: [u16]
}

impl CStr16 {
    pub fn from_literal(data: &'static [u16]) -> &'static CStr16 {
        return unsafe {&*(data as *const _ as *const _)};
    }

    pub fn as_ptr(&self) -> *const u16 {
        return &self.data[0]
    }
}

impl fmt::Debug for CStr16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;
        <Self as fmt::Display>::fmt(self, f)?;
        write!(f, "\"")
    }
}

impl fmt::Display for CStr16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for word in self.data.iter() {
            if *word != 0 {
                write!(f, "{}", *word as u8 as char)?;
            }
        }
        Ok(())
    }
}
