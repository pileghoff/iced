//! Show toggle controls using togglers.
use crate::backend::{self, Backend};
use crate::Renderer;
use iced_native::toggler;

pub use iced_style::toggler::{Style, StyleSheet};

/// A toggler that can be toggled.
///
/// This is an alias of an `iced_native` toggler with an `iced_wgpu::Renderer`.
pub type Toggler<Message, Backend> =
    iced_native::Toggler<Message, Renderer<Backend>>;

impl<B> toggler::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    const DEFAULT_SIZE: u16 = 20;
}
