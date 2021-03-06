use crate::error::BootError;
use efi::{gop::ModeInformation, BootServices, GraphicsOutputProtocol};
use yboot2_proto::{video::PixelFormat, LoadProtocol, VideoInfo};

// TODO: "Any" format
// TODO: "Text" format

fn pixel_to_efi(from: PixelFormat) -> Option<efi::gop::PixelFormat> {
    use efi::gop::PixelFormat::*;
    match from {
        PixelFormat::LfbRgb32 => Some(PixelRedGreenBlueReserved8BitPerColor),
        PixelFormat::LfbBgr32 => Some(PixelBlueGreenRedReserved8BitPerColor),
        _ => None,
    }
}

fn pixel_from_efi(from: efi::gop::PixelFormat) -> Option<PixelFormat> {
    use efi::gop::PixelFormat::*;
    match from {
        PixelRedGreenBlueReserved8BitPerColor => Some(PixelFormat::LfbRgb32),
        PixelBlueGreenRedReserved8BitPerColor => Some(PixelFormat::LfbBgr32),
    }
}

fn find_mode(
    proto: &GraphicsOutputProtocol,
    req: &VideoInfo,
) -> Result<(u32, &'static ModeInformation), BootError> {
    let req_format = pixel_to_efi(req.format).unwrap();
    for (num, info) in proto.mode_iter() {
        if info.horizontal_resolution == req.width
            && info.vertical_resolution == req.height
            && info.pixel_format == req_format
        {
            return Ok((num, info));
        }
    }
    Err(BootError::VideoModeUnsupported)
}

pub fn set_mode<T: LoadProtocol>(bs: &BootServices, data: &mut T) -> Result<(), BootError> {
    let gop = bs
        .locate_protocol::<GraphicsOutputProtocol>()
        .map_err(|_| BootError::VideoModeFailed)?;

    match find_mode(gop, data.get_video_info()) {
        Ok((num, info)) => {
            let mode = gop.set_mode(num).map_err(|_| BootError::VideoModeFailed)?;

            let info = VideoInfo {
                width: info.horizontal_resolution,
                height: info.vertical_resolution,
                format: pixel_from_efi(info.pixel_format).unwrap(),
                framebuffer: mode.framebuffer_addr() as u64,
                pitch: 4 * info.horizontal_resolution as u64,
            };

            data.set_video_info(&info);

            Ok(())
        }
        Err(err) => Err(err),
    }
}
