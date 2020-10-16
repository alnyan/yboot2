use core::fmt;

use efi;

#[derive(Debug)]
pub enum BootError {
    ImageLoadError(ImageLoadError),
    InitrdLoadError(InitrdLoadError),
    ProtocolError(ProtocolError),
    MemoryMapError(efi::Status),
    FileError(efi::Status),
    TerminateServicesError(efi::Status),
    VideoModeUnsupported,
    VideoModeFailed,
}

#[derive(Debug)]
pub enum ImageLoadError {
    BadAddress(u64, u64, u64),
    BadSegment(u64, u64, u64),
    IOError(efi::Status),
    NoProtocol,
    BadMagic,
    BadTarget,
}

#[derive(Debug)]
pub enum InitrdLoadError {
    IOError(efi::Status),
    NoSpace,
}

#[derive(Debug)]
pub enum ProtocolError {}

impl From<ProtocolError> for BootError {
    fn from(p: ProtocolError) -> Self {
        BootError::ProtocolError(p)
    }
}

impl From<InitrdLoadError> for BootError {
    fn from(p: InitrdLoadError) -> Self {
        BootError::InitrdLoadError(p)
    }
}

impl From<ImageLoadError> for BootError {
    fn from(p: ImageLoadError) -> Self {
        BootError::ImageLoadError(p)
    }
}

impl From<&BootError> for efi::Status {
    fn from(f: &BootError) -> Self {
        todo!()
    }
}

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BootError::*;
        match self {
            ImageLoadError(e) => e.fmt(f),
            InitrdLoadError(e) => e.fmt(f),
            _ => {
                write!(f, "Unknown error: {:?}", self)?;
                Ok(())
            }
        }
    }
}

impl fmt::Display for InitrdLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InitrdLoadError::*;
        match self {
            IOError(e) => write!(f, "I/O or file error (initrd): {:?}", e),
            NoSpace => write!(f, "Failed to fit initrd in memory"),
        }
    }
}

impl fmt::Display for ImageLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ImageLoadError::*;
        match self {
            BadAddress(addr, start, end) => write!(
                f,
                "Invalid image address: 0x{:016x}. Expected in range 0x{:016x} .. 0x{:016x}",
                addr, start, end
            ),
            BadSegment(page, start, end) => write!(
                f,
                "Invalid segment range: 0x{:016x} .. 0x{:016x}. Page 0x{:016x} can't be used.",
                start, end, page
            ),
            IOError(e) => write!(f, "I/O or file error (image): {:?}", e),
            NoProtocol => write!(f, "The image doesn't have a protocol structure"),
            BadTarget => write!(f, "The image targets a different arch"),
            BadMagic => write!(f, "Bad image magic"),
        }
    }
}
