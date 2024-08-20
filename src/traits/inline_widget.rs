use std::{convert::From, future::Future};

use teloxide::{
    dispatching::UpdateHandler,
    payloads::EditMessageReplyMarkupSetters,
    prelude::Requester,
    types::{ChatId, InlineKeyboardMarkup, MessageId, ReplyMarkup},
};

/// Trait that allows to combine inline_widgets together within the `user-defined` one and provides a way to handle a widget's logic
///
/// Don't implement it manually, it's more convenient to use the [`#[derive(InlineWidget)`] macro
pub trait InlineWidget {
    type Dialogue;
    type Bot: Sync + Requester;
    type Err: From<<Self::Bot as Requester>::Err>;

    /// Returns the [`dptree`]-handler schema for a `user-defined` widget
    fn schema() -> UpdateHandler<Self::Err>;

    /// Returns the [`InlineKeyboardMarkup`] for a `user-defined` widget
    fn inline_keyboard_markup(&self) -> InlineKeyboardMarkup;

    /// Updates the state of a `user-defined` widget
    fn update_state(
        self,
        dialogue: &Self::Dialogue,
    ) -> impl Future<Output = Result<(), Self::Err>> + Send;

    /// Redraws a `user-defined` widget
    fn redraw(
        &self,
        bot: &Self::Bot,
        chat_id: ChatId,
        message_id: MessageId,
    ) -> impl Future<Output = Result<(), Self::Err>> + Send
    where
        Self: Sync,
    {
        async move {
            bot.edit_message_reply_markup(chat_id, message_id)
                .reply_markup(self.inline_keyboard_markup())
                .await?;

            Ok(())
        }
    }

    fn reply_markup(&self) -> ReplyMarkup {
        ReplyMarkup::InlineKeyboard(self.inline_keyboard_markup())
    }

    // TODO empty_cell_icon
    //?
    fn empty_cell_icon() -> &'static str {
        "✖️"
    }
}
