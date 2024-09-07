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

/// Checkbox list widget
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckboxList<T> {
    /// Size of the [`CheckboxList`] widget
    pub size: Size,
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
    pub fn new(items: impl IntoIterator<Item = (bool, T)>, size: Size) -> Self {
        Self { items: Vec::from_iter(items), size }
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

    /// [`dptree`]-schema for the [`CheckboxList`] widget
    pub fn schema<W>(parameters: &'static CheckboxListSchemaParameters) -> UpdateHandler<W::Err>
    where
        W: 'static + Clone + Send + Sync + InlineWidget + WidgetContainer<Self>,
        W::Bot: 'static + Clone + Send + Sync,
        W::Dialogue: 'static + Clone + Send + Sync,
    {
        dptree::entry()
            .filter_map(move |cq: CallbackQuery| {
                Some(CheckboxListItemIndex(cq.data?.strip_prefix(parameters.prefix)?.parse().ok()?))
            })
            .filter_map(|cq: CallbackQuery| cq.message.map(|msg| (msg.chat.id, msg.id, cq.id)))
            .endpoint(
                |bot: W::Bot,
                 dialogue: W::Dialogue,
                 mut widget: W,
                 (chat_id, message_id, cq_id): (ChatId, MessageId, String),
                 widget_styles: WidgetStyles,
                 CheckboxListItemIndex(i): CheckboxListItemIndex| async move {
                    bot.answer_callback_query(cq_id).await?;

                    widget.get_widget().toggle(i);
                    // It's safe to update the view (keyboard) before the state if updates are
                    // processed consistently in a single chat, so there is no
                    // races
                    widget.redraw(&bot, chat_id, message_id, &widget_styles).await?;
                    widget.update_state(&dialogue).await?;

                    Ok(())
                },
            )
    }

    /// Creates the [`InlineKeyboardMarkup`] for a [`CheckboxList`] widget with
    /// specified callback query `prefix` and size.
    ///
    /// It's not supposed to be used directly
    pub fn inline_keyboard_markup(
        &self,
        parameters: &CheckboxListSchemaParameters,
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
            for (column_i, (active, item)) in row_chunk.iter().enumerate() {
                let i = (row_i * columns as usize) + column_i;
                let icon = if *active {
                    &styles.checkbox_list_style.active_icon
                } else {
                    &styles.checkbox_list_style.inactive_icon
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

impl<T> FromIterator<T> for CheckboxList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        CheckboxList::from(iter.into_iter().map(|item| (false, item)).collect::<Vec<(bool, T)>>())
    }
}

impl<T> FromIterator<(bool, T)> for CheckboxList<T> {
    fn from_iter<I: IntoIterator<Item = (bool, T)>>(iter: I) -> Self {
        CheckboxList::from(iter.into_iter().collect::<Vec<(bool, T)>>())
    }
}

impl<T> From<Vec<T>> for CheckboxList<T> {
    fn from(value: Vec<T>) -> Self {
        let size = Size::new(1, value.len() as u8);
        CheckboxList::new(value.into_iter().map(|item| (false, item)), size)
    }
}

impl<T> From<Vec<(bool, T)>> for CheckboxList<T> {
    fn from(value: Vec<(bool, T)>) -> Self {
        let size = Size::new(1, value.len() as u8);
        CheckboxList::new(value, size)
    }
}

impl<T> GetSize for CheckboxList<T> {
    fn size(&self) -> Size {
        self.size
    }
}

pub struct CheckboxListSchemaParameters {
    pub prefix: &'static str,
    pub noop_data: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkbox_list() {
        let mut cl = CheckboxList::new([(false, 1), (false, 2), (true, 3)], Size::new(1, 3));
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
        let mut cl: CheckboxList<i32> = CheckboxList::new([], Size::new(0, 0));

        cl.toggle(1);
    }
}
