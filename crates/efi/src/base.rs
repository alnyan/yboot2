use core::ffi::c_void;

pub type Handle = *mut c_void;

#[derive(Eq, PartialEq)]
pub enum Status {
    Success,
    Err,
    BufferTooSmall
}

const EFI_SUCCESS:          u64 = 0;
const EFI_ERR:              u64 = 0x8000000000000000;
const EFI_BUFFER_TOO_SMALL: u64 = EFI_ERR | 0x05;

impl Status {
    pub fn to_str(self) -> &'static str {
        match self {
            Status::Success         => "EFI_SUCCESS",
            Status::Err             => "EFI_ERR",
            Status::BufferTooSmall  => "EFI_BUFFER_TOO_SMALL",
        }
    }

    pub fn to_num(self) -> u64 {
        match self {
            Status::Success         =>      EFI_SUCCESS,
            Status::Err             =>      EFI_ERR,
            Status::BufferTooSmall  =>      EFI_BUFFER_TOO_SMALL,
        }
    }
    pub fn from_num(v: u64) -> Status {
        match v {
            0                       => Status::Success,
            EFI_ERR                 => Status::Err,
            EFI_BUFFER_TOO_SMALL    => Status::BufferTooSmall,
            _                       => Status::Err
        }
    }
}
