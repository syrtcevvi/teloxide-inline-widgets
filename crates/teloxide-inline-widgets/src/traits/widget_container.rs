/// Is used to allow `user-defined` widget to retrieve the inner components for
/// the `state management`
pub trait WidgetContainer<W> {
    /// Returns the mutable reference to an item
    fn get_widget(&mut self) -> &mut W;
}
