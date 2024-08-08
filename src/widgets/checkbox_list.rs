mod checkbox_list_item_index;
mod settings;

use std::{
    convert::From,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use smallvec::SmallVec;
use teloxide::{
    dispatching::UpdateHandler,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, MessageId, ReplyMarkup},
};

use serde::{Deserialize, Serialize};

use self::checkbox_list_item_index::CheckboxListItemIndex;
pub use self::settings::CheckboxListSettings;
use crate::traits::{Component, UserDefinedWidget, WidgetContainer};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckboxList<T, S> {
    // Maybe simple Vec?
    items: SmallVec<[(bool, T); 4]>,
    #[serde(skip)]
    settings: PhantomData<S>,
}

impl<T, S> CheckboxList<T, S>
where
    T: Display,
    S: CheckboxListSettings,
{
    pub fn new(items: impl IntoIterator<Item = T>) -> Self {
        Self {
            items: SmallVec::from_iter(items.into_iter().map(|i| (false, i))),
            settings: PhantomData,
        }
    }

    pub fn toggle(&mut self, i: usize) {
        // TODO limits?
        self.items[i].0 = !self.items[i].0;
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
                    if data.starts_with(S::prefix()) {
                        return Some(CheckboxListItemIndex(
                            data[S::prefix().len()..].parse().ok()?,
                        ));
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
                 CheckboxListItemIndex(i): CheckboxListItemIndex| async move {
                    bot.answer_callback_query(cq_id).await?;

                    widget.get_widget().toggle(i);
                    // It's safe to update the view (keyboard) before the state if updates are processed
                    // consistently in a single chat, so there is no races
                    widget.redraw(&bot, chat_id, message_id).await?;
                    widget.update_state(&dialogue).await?;

                    Ok(())
                },
            )
    }

    fn inline_keyboard_markup(&self) -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::with_capacity(S::size().0 as usize);

        for (row_i, chunk) in self.items.chunks(S::size().1 as usize).enumerate() {
            let row = chunk
                .iter()
                .enumerate()
                .map(|(column_i, (active, item))| {
                    let i = (row_i * S::size().1 as usize) + column_i;
                    let icon = if *active {
                        S::active_icon()
                    } else {
                        S::inactive_icon()
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

impl<T, S> From<&CheckboxList<T, S>> for ReplyMarkup
where
    T: Display,
    S: CheckboxListSettings,
{
    fn from(value: &CheckboxList<T, S>) -> Self {
        ReplyMarkup::InlineKeyboard(value.inline_keyboard_markup())
    }
}

impl<T, S> Component for CheckboxList<T, S>
where
    T: Display,
    S: CheckboxListSettings,
{
    fn size(&self) -> (u8, u8) {
        S::size()
    }

    fn keyboard(&self) -> Vec<Vec<InlineKeyboardButton>> {
        self.inline_keyboard_markup().inline_keyboard
    }
}
