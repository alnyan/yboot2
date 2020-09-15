pub mod stop;
pub mod stip;
pub mod gop;
pub mod lip;
pub mod dpp;
pub mod sfsp;
pub mod fp;

pub trait Protocol {
    const GUID: super::Guid;
    //fn guid() -> &'static super::Guid;
}

pub use stop::SimpleTextOutputProtocol;
pub use stip::SimpleTextInputProtocol;
pub use gop::GraphicsOutputProtocol;
pub use lip::LoadedImageProtocol;
pub use dpp::DevicePathProtocol;
pub use sfsp::SimpleFileSystemProtocol;
pub use fp::{FileProtocol, File};
