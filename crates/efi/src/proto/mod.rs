pub mod stop;
pub mod stip;
pub mod gop;

pub trait Protocol {
    const GUID: super::Guid;
    //fn guid() -> &'static super::Guid;
}

pub use stop::SimpleTextOutputProtocol;
pub use stip::SimpleTextInputProtocol;
pub use gop::GraphicsOutputProtocol;
