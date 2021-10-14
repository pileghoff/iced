use crate::Renderer;

/// A container that distributes its contents vertically.
pub type Column<'a, Message, Backend> =
    iced_native::Column<'a, Message, Renderer<Backend>>;
