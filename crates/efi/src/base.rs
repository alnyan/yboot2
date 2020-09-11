use core::ffi::c_void;

pub type Handle = *mut c_void;

#[derive(Eq, PartialEq, Debug)]
pub enum Status {
    Success,
    Err,
    InvalidParameter,
    BufferTooSmall
}

const EFI_SUCCESS:                  u64 = 0;
const EFI_ERR:                      u64 = 0x8000000000000000;
const EFI_INVALID_PARAMETER:        u64 = EFI_ERR | 0x02;
const EFI_BUFFER_TOO_SMALL:         u64 = EFI_ERR | 0x05;

impl Status {
    pub fn to_num(self) -> u64 {
        match self {
            Status::Success             => EFI_SUCCESS,
            Status::Err                 => EFI_ERR,
            Status::InvalidParameter    => EFI_INVALID_PARAMETER,
            Status::BufferTooSmall      => EFI_BUFFER_TOO_SMALL,
        }
    }
    pub fn from_num(v: u64) -> Status {
        match v {
            0                       => Status::Success,
            EFI_ERR                 => Status::Err,
            EFI_INVALID_PARAMETER   => Status::InvalidParameter,
            EFI_BUFFER_TOO_SMALL    => Status::BufferTooSmall,
            _                       => Status::Err
        }
    }
}

pub struct Guid {
    pub data1:  u32,
    pub data2:  u16,
    pub data3:  u16,
    pub data4:  [u8; 8]
}
