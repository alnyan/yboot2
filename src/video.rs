use yboot2_proto::{LoadProtocol, VideoRequest, VideoInfo, PixelFormat};
use efi::{BootServices, Status, GraphicsOutputProtocol, gop::ModeInformation};

// TODO: "Any" format
// TODO: "Text" format

fn pixel_to_efi(from: PixelFormat) -> Option<efi::gop::PixelFormat> {
    use efi::gop::PixelFormat::*;
    match from {
        PixelFormat::LfbRgb32   => Some(PixelRedGreenBlueReserved8BitPerColor),
        PixelFormat::LfbBgr32   => Some(PixelBlueGreenRedReserved8BitPerColor),
        _                       => None
    }
}

fn pixel_from_efi(from: efi::gop::PixelFormat) -> Option<PixelFormat> {
    use efi::gop::PixelFormat::*;
    match from {
        PixelRedGreenBlueReserved8BitPerColor   => Some(PixelFormat::LfbRgb32),
        PixelBlueGreenRedReserved8BitPerColor   => Some(PixelFormat::LfbBgr32),
        _                                       => None
    }
}

fn find_mode(proto: &GraphicsOutputProtocol,
             req: &VideoRequest) -> Result<(u32, &'static ModeInformation), Status> {
    let req_format = pixel_to_efi(req.format).unwrap();
    for (num, info) in proto.mode_iter() {
        if info.horizontal_resolution == req.width &&
           info.vertical_resolution == req.height &&
           info.pixel_format == req_format {
            return Ok((num, info));
        }
    }
    Err(Status::InvalidParameter)
}

pub fn set_mode<T: LoadProtocol>(bs: &BootServices, data: &mut T) -> Result<(), Status> {
    let req = data.get_video_request();
    let gop = bs.locate_protocol::<GraphicsOutputProtocol>()?;

    match find_mode(gop, &req) {
        Ok((num, info))     => {
            let mode = gop.set_mode(num)?;

            let info = VideoInfo {
                width:          info.horizontal_resolution,
                height:         info.vertical_resolution,
                format:         pixel_from_efi(info.pixel_format).unwrap(),
                framebuffer:    mode.framebuffer_addr(),
                pitch:          4 * info.horizontal_resolution as usize
            };

            data.set_video_info(&info);

            Ok(())
        },
        Err(err)            => {
            Err(err)
        }
    }
}
