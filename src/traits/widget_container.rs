pub trait WidgetContainer<W> {
    fn get_widget(&mut self) -> &mut W;
}
