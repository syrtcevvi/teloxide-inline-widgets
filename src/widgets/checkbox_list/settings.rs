pub trait CheckboxListSettings {
    fn prefix() -> &'static str;

    // fn limits

    // better types? (8 max in a row) or simply assert?
    fn size() -> (u8, u8);

    // fn order() -> Order {
    //     Order::RowMajor
    // }

    fn active_icon() -> &'static str {
        "☑"
    }

    fn inactive_icon() -> &'static str {
        "☐"
    }
}
