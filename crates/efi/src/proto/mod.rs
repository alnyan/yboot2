pub mod stop;
pub mod gop;

pub trait Protocol {
    fn guid() -> &'static super::Guid;
}

pub use stop::SimpleTextOutputProtocol;
pub use gop::GraphicsOutputProtocol;
