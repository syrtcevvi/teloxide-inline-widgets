use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

/// Single inline-keyboard `callback query` button
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Button {
    /// Text that is displayed within a button
    pub label: String,
}

impl Button {
    /// Creates new [`Button`] instance with provided label
    pub fn new(label: &str) -> Self {
        Self { label: label.to_owned() }
    }

    /// Creates the [`InlineKeyboardMarkup`] for a [`Button`] widget with
    /// specified callback query `data`
    ///
    /// It's not supposed to be used directly
    pub fn inline_keyboard_markup(&self, data: &'static str) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(&self.label, data)]])
    }
}
