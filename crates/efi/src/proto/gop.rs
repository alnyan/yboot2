use crate::{Status, Guid};
use crate::proto::Protocol;
use core::convert::TryFrom;
use core::ffi::c_void;

#[repr(C)]
pub struct Mode {
    pub max_mode:           u32,
    pub mode:               u32,
    pub info:               *mut c_void,
    pub size_of_info:       usize,
    pub framebuffer_base:   usize,
    pub framebuffer_size:   usize
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PixelFormat {
    PixelRedGreenBlueReserved8BitPerColor,
    PixelBlueGreenRedReserved8BitPerColor
}

#[repr(C)]
pub struct PixelBitmask {
    red_mask:       u32,
    green_mask:     u32,
    blue_mask:      u32,
    reserved_mask:  u32
}

#[repr(C)]
pub struct ModeInformation {
    pub version:                u32,
    pub horizontal_resolution:  u32,
    pub vertical_resolution:    u32,
    pub pixel_format:           PixelFormat,
    pub pixel_information:      PixelBitmask,
    pub pixels_per_scanline:    u32
}

#[repr(C)]
pub struct GraphicsOutputProtocol {
    query_mode:         unsafe fn (*const GraphicsOutputProtocol,
                                   u32,
                                   *mut usize,
                                   *mut *mut ModeInformation) -> u64,
    set_mode:           unsafe fn (*mut GraphicsOutputProtocol,
                                   u32) -> u64,
    blt:                *mut c_void,
    mode:               &'static Mode
}

pub struct ModeIterator<'a> {
    protocol:   &'a GraphicsOutputProtocol,
    number:     u32,
    max_number: u32,
}

impl Protocol for GraphicsOutputProtocol {
    const GUID: Guid = Guid {
        data1:  0x9042a9de,
        data2:  0x23dc,
        data3:  0x4a38,
        data4:  [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a]
    };
}

impl GraphicsOutputProtocol {
    pub fn mode_iter(&self) -> ModeIterator {
        return ModeIterator {
            protocol:   self,
            number:     0,
            max_number: self.mode.max_mode
        };
    }

    pub fn set_mode(&mut self, num: u32) -> Result<&'static Mode, Status> {
        match Status::from(unsafe {
            (self.set_mode)(self as *mut GraphicsOutputProtocol, num)
        }) {
            Status::Success => Ok(self.mode),
            err             => Err(err)
        }
    }

    // TODO: pixel format
    pub fn find_mode(&self, width: u32, height: u32) -> Option<u32> {
        for (num, mode) in self.mode_iter() {
            if mode.horizontal_resolution == width && mode.vertical_resolution == height {
                return Some(num);
            }
        }
        return None;
    }
}

impl<'a> Iterator for ModeIterator<'a> {
    type Item = (u32, &'static ModeInformation);

    fn next(&mut self) -> Option<Self::Item> {
        if self.number == self.max_number {
            return None
        }

        let mut mode: *mut ModeInformation = core::ptr::null_mut();
        let mut junk: usize = 0;

        match Status::from(unsafe {
            (self.protocol.query_mode)(
                self.protocol as *const GraphicsOutputProtocol,
                self.number,
                (&mut junk) as *mut usize,
                (&mut mode) as *mut *mut ModeInformation
            )
        }) {
            Status::Success => {
                self.number += 1;
                unsafe {mode.as_ref()}.map(|x| { (self.number - 1, x) })
            },
            _ => None
        }
    }
}

impl Mode {
    pub fn framebuffer(&self) -> &mut [u32] {
        return unsafe {
            core::slice::from_raw_parts_mut(self.framebuffer_base as *mut u32, self.framebuffer_size)
        }
    }

    pub fn framebuffer_addr(&self) -> usize {
        self.framebuffer_base
    }
}

impl TryFrom<u32> for PixelFormat {
    type Error = ();

    fn try_from(f: u32) -> Result<Self, Self::Error> {
        match f {
            0   => Ok(PixelFormat::PixelRedGreenBlueReserved8BitPerColor),
            1   => Ok(PixelFormat::PixelBlueGreenRedReserved8BitPerColor),
            _   => Err(())
        }
    }
}
