use crate::types::Size;

pub trait GetSize {
    /// Returns the size of the widget
    fn size(&self) -> Size;
}
