use std::fmt::Display;

use serde::{Deserialize, Serialize};
use teloxide::{
    dispatching::UpdateHandler,
    dptree,
    prelude::Requester,
    types::{CallbackQuery, ChatId, InlineKeyboardButton, InlineKeyboardMarkup, MessageId},
};

use crate::{
    traits::{GetSize, InlineWidget, WidgetContainer},
    types::{Size, WidgetStyles},
};

/// Radio list widget
// FIXME add gif to docs?
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RadioList<T> {
    pub size: Size,
    items: Vec<T>,
    active_item_i: Option<usize>,
}

/// Index of a [`RadioList`] item. Used as a unique type in the
/// [`dptree`]-handler schema
#[derive(Debug, Clone)]
struct RadioListItemIndex(pub usize);

impl<T> RadioList<T> {
    /// Creates new [`RadioList`] instance from a collection with optionally
    /// active item.
    ///
    /// Panics if the `active_item_i` index is out of bounds
    pub fn new(
        items: impl IntoIterator<Item = T>,
        active_item_i: Option<usize>,
        size: Size,
    ) -> Self {
        let items = Vec::from_iter(items);

        if items.is_empty() {
            log::warn!("RadioList is empty");
        }

        if let Some(i) = active_item_i {
            assert!(i < items.len());
        }

        Self { items: Vec::from_iter(items), active_item_i, size }
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
    /// [`dptree`]-schema for the [`RadioList`] widget
    pub fn schema<W>(parameters: &'static RadioListSchemaParameters) -> UpdateHandler<W::Err>
    where
        W: 'static + Clone + Send + Sync + InlineWidget + WidgetContainer<Self>,
        W::Bot: 'static + Clone + Send + Sync,
        W::Dialogue: 'static + Clone + Send + Sync,
    {
        dptree::entry()
            .filter_map(move |cq: CallbackQuery| {
                Some(RadioListItemIndex(cq.data?.strip_prefix(parameters.prefix)?.parse().ok()?))
            })
            .filter_map(|cq: CallbackQuery| cq.message.map(|msg| (msg.chat.id, msg.id, cq.id)))
            .endpoint(
                |bot: W::Bot,
                 dialogue: W::Dialogue,
                 mut widget: W,
                 (chat_id, message_id, cq_id): (ChatId, MessageId, String),
                 widget_styles: WidgetStyles,
                 RadioListItemIndex(i): RadioListItemIndex| async move {
                    bot.answer_callback_query(cq_id).await?;

                    let rl = widget.get_widget();
                    if rl.active_item_i == Some(i) {
                        log::warn!("User clicked on the already selected radio button");
                        return Ok(());
                    }

                    rl.set_active(i);
                    // FIXME: Probably allow some callback here? Or after

                    // It's safe to update the view (keyboard) before the state if updates are
                    // processed consistently in a single chat, so there is no
                    // races
                    widget.redraw(&bot, chat_id, message_id, &widget_styles).await?;
                    widget.update_state(&dialogue).await?;

                    Ok(())
                },
            )
    }

    /// Creates the [`InlineKeyboardMarkup`] for a [`RadioList`] widget with
    /// specified callback query `prefix` and size.
    ///
    /// It's not supposed to be used directly
    pub fn inline_keyboard_markup(
        &self,
        parameters: &RadioListSchemaParameters,
        styles: &WidgetStyles,
    ) -> InlineKeyboardMarkup
    where
        T: Display,
    {
        let Size { rows, columns } = self.size;

        use std::iter::repeat;
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = repeat(
            repeat(InlineKeyboardButton::callback(
                styles.common_style.empty_cell_icon.clone(),
                parameters.noop_data,
            ))
            .take(columns as usize)
            .collect(),
        )
        .take(rows as usize)
        .collect();

        for (row_i, row_chunk) in self.items.chunks(columns as usize).enumerate() {
            for (column_i, item) in row_chunk.iter().enumerate() {
                let i = (row_i * columns as usize) + column_i;
                let icon = if self.active_item_i == Some(i) {
                    &styles.radio_list_style.active_icon
                } else {
                    &styles.radio_list_style.inactive_icon
                };

                keyboard[row_i][column_i] = InlineKeyboardButton::callback(
                    format!("{icon} {item}"),
                    format!("{}{}", parameters.prefix, i),
                )
            }
        }

        InlineKeyboardMarkup::new(keyboard)
    }
}

impl<T> From<Vec<T>> for RadioList<T> {
    fn from(value: Vec<T>) -> Self {
        let size = Size::new(1, value.len() as u8);
        RadioList::new(value, None, size)
    }
}

impl<T> GetSize for RadioList<T> {
    fn size(&self) -> Size {
        self.size
    }
}

pub struct RadioListSchemaParameters {
    pub prefix: &'static str,
    pub noop_data: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radio_list() {
        let mut rl = RadioList::new([1, 2], None, Size::new(1, 2));
        assert!(rl.active_item().is_none());
        rl.set_active(1);
        assert_eq!(rl.active_item(), Some(&2));
    }

    #[test]
    #[should_panic]
    fn active_item_i_out_of_bounds() {
        let _rl: RadioList<i32> = RadioList::new([1, 2, 3], Some(3), Size::new(1, 3));
    }

    #[test]
    #[should_panic]
    fn i_out_of_bounds() {
        let mut rl = RadioList::new([1, 2, 3], None, Size::new(1, 3));

        rl.set_active(3);
    }
}
