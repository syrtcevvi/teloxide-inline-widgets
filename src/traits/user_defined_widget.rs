use std::{convert::From, future::Future};

use teloxide::{
    dispatching::UpdateHandler,
    payloads::EditMessageReplyMarkupSetters,
    prelude::Requester,
    types::{ChatId, InlineKeyboardMarkup, MessageId, ReplyMarkup},
};

pub trait UserDefinedWidget {
    type Dialogue;
    type Bot: Sync + Requester;
    type Err: From<<Self::Bot as Requester>::Err>;

    fn schema() -> UpdateHandler<Self::Err>;

    fn inline_keyboard_markup(&self) -> InlineKeyboardMarkup;

    fn update_state(
        self,
        dialogue: &Self::Dialogue,
    ) -> impl Future<Output = Result<(), Self::Err>> + Send;

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

    fn empty_cell_icon() -> &'static str {
        "✖️"
    }
}
