use crate::proto::{LoadProtocol, VideoRequest, VideoInfo};
use efi::{BootServices, Status, GraphicsOutputProtocol, gop::ModeInformation};

fn find_mode(proto: &GraphicsOutputProtocol,
             req: &VideoRequest) -> Result<(u32, &'static ModeInformation), Status> {
    for (num, info) in proto.mode_iter() {
        if info.horizontal_resolution == req.width &&
           info.vertical_resolution == req.height &&
           info.pixel_format == req.format {
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
                format:         info.pixel_format,
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
