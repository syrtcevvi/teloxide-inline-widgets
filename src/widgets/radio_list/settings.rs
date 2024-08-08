pub trait RadioListSettings {
    fn prefix() -> &'static str;

    // fn layout() ->

    // fn limits

    // fn size() -> (u8, u8);

    // fn order() -> Order {
    //     Order::RowMajor
    // }
    fn size() -> (u8, u8);

    fn active_icon() -> &'static str {
        "🟢"
    }

    fn inactive_icon() -> Option<&'static str> {
        None
    }
}
