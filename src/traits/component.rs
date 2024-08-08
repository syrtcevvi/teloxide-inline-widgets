use teloxide::types::InlineKeyboardButton;

pub trait Component {
    fn size(&self) -> (u8, u8);

    fn keyboard(&self) -> Vec<Vec<InlineKeyboardButton>>;
}
