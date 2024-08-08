mod radio_list_item_index;
mod settings;

use std::{
    convert::From,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use teloxide::{
    dispatching::UpdateHandler,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, MessageId, ReplyMarkup},
};

use crate::traits::{Component, UserDefinedWidget, WidgetContainer};
use radio_list_item_index::RadioListItemIndex;
pub use settings::RadioListSettings;

/// Radio list widget
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RadioList<T, S> {
    // Maybe simple Vec?
    items: SmallVec<[T; 4]>,
    active_item_i: Option<usize>,
    #[serde(skip)]
    settings: PhantomData<S>,
}

impl<T, S> RadioList<T, S>
where
    T: Display,
    S: RadioListSettings,
{
    pub fn new(items: impl IntoIterator<Item = T>, active_item_i: Option<usize>) -> Self {
        Self {
            items: SmallVec::from_iter(items),
            active_item_i,
            settings: PhantomData,
        }
    }

    pub fn active_item(&self) -> Option<&T> {
        if let Some(active_item_i) = self.active_item_i {
            return Some(&self.items[active_item_i]);
        }
        None
    }

    // FIXME
    pub fn set_active(&mut self, i: usize) -> bool {
        let active_changed = self.active_item_i == Some(i);

        self.active_item_i = Some(i);
        active_changed
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn schema<W>() -> UpdateHandler<W::Err>
    where
        W: 'static + Clone + Send + Sync + UserDefinedWidget + WidgetContainer<Self>,
        W::Bot: 'static + Clone + Send + Sync,
        W::Dialogue: 'static + Clone + Send + Sync,
    {
        dptree::entry()
            .filter_map(|cq: CallbackQuery| {
                if let Some(data) = cq.data {
                    // TODO ignore if index doesn't change
                    if data.starts_with(S::prefix()) {
                        return Some(RadioListItemIndex(data[S::prefix().len()..].parse().ok()?));
                    }
                }
                None
            })
            .filter_map(|cq: CallbackQuery| cq.message.map(|msg| (msg.chat.id, msg.id, cq.id)))
            .endpoint(
                |bot: W::Bot,
                 (chat_id, message_id, cq_id): (ChatId, MessageId, String),
                 dialogue: W::Dialogue,
                 mut widget: W,
                 RadioListItemIndex(i): RadioListItemIndex| async move {
                    bot.answer_callback_query(cq_id).await?;

                    widget.get_widget().set_active(i);
                    // FIXME: Probably allow some callback here? Or after

                    // It's safe to update the view (keyboard) before the state if updates are processed
                    // consistently in a single chat, so there is no races
                    widget.redraw(&bot, chat_id, message_id).await?;
                    widget.update_state(&dialogue).await?;

                    Ok(())
                },
            )
    }

    pub fn inline_keyboard_markup(&self) -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::with_capacity(S::size().0 as usize);

        // TODO order (RowMajor, Column Major)?
        for (row_i, chunk) in self.items.chunks(S::size().1 as usize).enumerate() {
            let row = chunk
                .iter()
                .enumerate()
                .map(|(column_i, item)| {
                    let i = (row_i * S::size().1 as usize) + column_i;
                    let icon = if self.active_item_i == Some(i) {
                        S::active_icon()
                    } else {
                        S::inactive_icon().unwrap_or("")
                    };

                    InlineKeyboardButton::callback(
                        format!("{} {}", icon, item),
                        format!("{}{}", S::prefix(), i),
                    )
                })
                .collect();

            keyboard.push(row);
        }

        InlineKeyboardMarkup::new(keyboard)
    }
}

impl<T, S> From<&RadioList<T, S>> for ReplyMarkup
where
    T: Display,
    S: RadioListSettings,
{
    fn from(value: &RadioList<T, S>) -> Self {
        ReplyMarkup::InlineKeyboard(value.inline_keyboard_markup())
    }
}

impl<T, S> Component for RadioList<T, S>
where
    T: Display,
    S: RadioListSettings,
{
    fn size(&self) -> (u8, u8) {
        S::size()
    }

    fn keyboard(&self) -> Vec<Vec<InlineKeyboardButton>> {
        self.inline_keyboard_markup().inline_keyboard
    }
}
