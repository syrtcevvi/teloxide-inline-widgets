use std::fmt::Display;

use serde::{Deserialize, Serialize};
use teloxide::{
    dispatching::UpdateHandler,
    dptree,
    prelude::Requester,
    types::{CallbackQuery, ChatId, InlineKeyboardButton, InlineKeyboardMarkup, MessageId},
};

use crate::traits::{InlineWidget, WidgetContainer};

/// Radio list widget
// FIXME add gif to docs?
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RadioList<T> {
    items: Vec<T>,
    active_item_i: Option<usize>,
}

/// Index of a [`RadioList`] item. Used as a unique type in the [`dptree`]-handler schema
#[derive(Debug, Clone)]
struct RadioListItemIndex(pub usize);

impl<T> RadioList<T> {
    /// Creates new [`RadioList`] instance from a collection with optionally active item.
    ///
    /// Panics if the `active_item_i` index is out of bounds
    pub fn new(items: impl IntoIterator<Item = T>, active_item_i: Option<usize>) -> Self {
        let items = Vec::from_iter(items);

        if items.is_empty() {
            log::warn!("RadioList is empty");
        }

        if let Some(i) = active_item_i {
            assert!(i < items.len());
        }

        Self {
            items: Vec::from_iter(items),
            active_item_i,
        }
    }

    /// Returns the reference to the active item
    pub fn active_item(&self) -> Option<&T> {
        self.active_item_i.map(|i| &self.items[i])
    }

    /// Returns the index of the active item
    pub fn active_item_i(&self) -> Option<usize> {
        self.active_item_i
    }

    /// Sets the active item by index
    ///
    /// Panics if the index is out of bounds
    pub fn set_active(&mut self, i: usize) {
        assert!(i < self.items.len());

        self.active_item_i = Some(i);
    }

    /// Returns the slice of items contained within the [`RadioList`]
    pub fn items(&self) -> &[T] {
        &self.items
    }

    // TODO more helpful functions

    // TODO Add tests
    pub fn schema<W>(prefix: &'static str) -> UpdateHandler<W::Err>
    where
        W: 'static + Clone + Send + Sync + InlineWidget + WidgetContainer<Self>,
        W::Bot: 'static + Clone + Send + Sync,
        W::Dialogue: 'static + Clone + Send + Sync,
    {
        dptree::entry()
            .filter_map(move |cq: CallbackQuery| {
                Some(RadioListItemIndex(
                    cq.data?.strip_prefix(prefix)?.parse().ok()?,
                ))
            })
            .filter_map(|cq: CallbackQuery| cq.message.map(|msg| (msg.chat.id, msg.id, cq.id)))
            .endpoint(
                |bot: W::Bot,
                 (chat_id, message_id, cq_id): (ChatId, MessageId, String),
                 dialogue: W::Dialogue,
                 mut widget: W,
                 RadioListItemIndex(i): RadioListItemIndex| async move {
                    bot.answer_callback_query(cq_id).await?;

                    let rl = widget.get_widget();
                    if rl.active_item_i == Some(i) {
                        log::warn!("User clicked on the already selected radio button");
                        return Ok(());
                    }

                    rl.set_active(i);
                    // FIXME: Probably allow some callback here? Or after

                    // It's safe to update the view (keyboard) before the state if updates are processed
                    // consistently in a single chat, so there is no races
                    widget.redraw(&bot, chat_id, message_id).await?;
                    widget.update_state(&dialogue).await?;

                    Ok(())
                },
            )
    }

    /// Creates the [`InlineKeyboardMarkup`] for a [`RadioList`] widget with specified
    /// `prefix` and size.
    ///
    /// It's not supposed to be used directly
    pub fn inline_keyboard_markup(
        &self,
        prefix: &str,
        (rows, columns): (u8, u8),
    ) -> InlineKeyboardMarkup
    where
        T: Display,
    {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::with_capacity(rows as usize);

        // TODO order (RowMajor, Column Major)?
        for (row_i, chunk) in self.items.chunks(columns as usize).enumerate() {
            let row = chunk
                .iter()
                .enumerate()
                .map(|(column_i, item)| {
                    let i = (row_i * columns as usize) + column_i;
                    let icon = if self.active_item_i == Some(i) {
                        // TODO parameter?
                        "🟢"
                    } else {
                        ""
                    };

                    InlineKeyboardButton::callback(format!("{icon} {item}"), format!("{prefix}{i}"))
                })
                .collect();

            keyboard.push(row);
        }

        InlineKeyboardMarkup::new(keyboard)
    }
}

impl<T> From<Vec<T>> for RadioList<T> {
    fn from(value: Vec<T>) -> Self {
        RadioList::new(value, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radio_list() {
        let mut rl = RadioList::new([1, 2], None);
        assert!(rl.active_item().is_none());
        rl.set_active(1);
        assert_eq!(rl.active_item(), Some(&2));
    }

    #[test]
    #[should_panic]
    fn active_item_i_out_of_bounds() {
        let _rl: RadioList<i32> = RadioList::new([1, 2, 3], Some(3));
    }

    #[test]
    #[should_panic]
    fn i_out_of_bounds() {
        let mut rl = RadioList::new([1, 2, 3], None);

        rl.set_active(3);
    }
}
