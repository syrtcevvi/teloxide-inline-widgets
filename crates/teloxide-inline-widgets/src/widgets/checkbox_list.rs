use std::fmt::Display;

use serde::{Deserialize, Serialize};
use teloxide::{
    dispatching::UpdateHandler,
    dptree,
    prelude::Requester,
    types::{CallbackQuery, ChatId, InlineKeyboardButton, InlineKeyboardMarkup, MessageId},
};

use crate::traits::{InlineWidget, WidgetContainer};

/// Checkbox list widget
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckboxList<T> {
    items: Vec<(bool, T)>,
}

#[derive(Debug, Clone)]
pub struct CheckboxListItemIndex(pub usize);

impl<T> CheckboxList<T> {
    /// Creates new [`CheckboxList`] instance from a collection of (`bool`, `T`)
    /// items.
    ///
    /// If you want to create an instance with selected values, pass _true_ with
    /// these values.
    pub fn new(items: impl IntoIterator<Item = (bool, T)>) -> Self {
        Self { items: Vec::from_iter(items) }
    }

    /// Toggles the selection of the item specified by the index
    ///
    /// Panics if the index is out of bounds
    pub fn toggle(&mut self, i: usize) {
        assert!(i < self.items.len());

        self.items[i].0 = !self.items[i].0;
    }

    /// Returns the iterator over the selected items in the [`CheckboxList`]
    pub fn selected_items(&self) -> impl Iterator<Item = &T> {
        self.items.iter().filter_map(|(selected, item)| if *selected { Some(item) } else { None })
    }

    pub fn schema<W>(prefix: &'static str) -> UpdateHandler<W::Err>
    where
        W: 'static + Clone + Send + Sync + InlineWidget + WidgetContainer<Self>,
        W::Bot: 'static + Clone + Send + Sync,
        W::Dialogue: 'static + Clone + Send + Sync,
    {
        dptree::entry()
            .filter_map(move |cq: CallbackQuery| {
                Some(CheckboxListItemIndex(cq.data?.strip_prefix(prefix)?.parse().ok()?))
            })
            .filter_map(|cq: CallbackQuery| cq.message.map(|msg| (msg.chat.id, msg.id, cq.id)))
            .endpoint(
                |bot: W::Bot,
                 (chat_id, message_id, cq_id): (ChatId, MessageId, String),
                 dialogue: W::Dialogue,
                 mut widget: W,
                 CheckboxListItemIndex(i): CheckboxListItemIndex| async move {
                    bot.answer_callback_query(cq_id).await?;

                    widget.get_widget().toggle(i);
                    // It's safe to update the view (keyboard) before the state if updates are
                    // processed consistently in a single chat, so there is no
                    // races
                    widget.redraw(&bot, chat_id, message_id).await?;
                    widget.update_state(&dialogue).await?;

                    Ok(())
                },
            )
    }

    pub fn inline_keyboard_markup(
        &self,
        prefix: &'static str,
        (rows, columns): (u8, u8),
    ) -> InlineKeyboardMarkup
    where
        T: Display,
    {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::with_capacity(rows as usize);

        for (row_i, chunk) in self.items.chunks(columns as usize).enumerate() {
            let row = chunk
                .iter()
                .enumerate()
                .map(|(column_i, (active, item))| {
                    let i = (row_i * columns as usize) + column_i;
                    let icon = if *active { "☑" } else { "☐" };

                    InlineKeyboardButton::callback(format!("{icon} {item}"), format!("{prefix}{i}"))
                })
                .collect();

            keyboard.push(row);
        }

        InlineKeyboardMarkup::new(keyboard)
    }
}

impl<T> From<Vec<T>> for CheckboxList<T> {
    fn from(value: Vec<T>) -> Self {
        CheckboxList::new(value.into_iter().map(|item| (false, item)))
    }
}

impl<T> From<Vec<(bool, T)>> for CheckboxList<T> {
    fn from(value: Vec<(bool, T)>) -> Self {
        CheckboxList::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkbox_list() {
        let mut cl = CheckboxList::new([(false, 1), (false, 2), (true, 3)]);
        assert_eq!(cl.selected_items().collect::<Vec<_>>(), [&3]);

        cl.toggle(2);
        assert_eq!(cl.selected_items().count(), 0);

        cl.toggle(0);
        cl.toggle(1);

        assert_eq!(cl.selected_items().collect::<Vec<_>>(), [&1, &2]);
    }

    #[test]
    #[should_panic]
    fn i_out_of_bounds() {
        let mut cl: CheckboxList<i32> = CheckboxList::new([]);

        cl.toggle(1);
    }
}
