use core::ffi::c_void;

pub type Handle = *mut c_void;

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
