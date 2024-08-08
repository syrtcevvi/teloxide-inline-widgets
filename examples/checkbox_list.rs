/*
    This example demonstrates how to use the `CheckboxList` widget.
*/
use std::future::Future;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::InlineKeyboardMarkup};
use teloxide_inline_widgets::prelude::*;

type Bot = teloxide::Bot;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type HandlerResult = Result<(), Error>;
type UpdateHandler = teloxide::dispatching::UpdateHandler<Error>;
type Storage = InMemStorage<State>;
type Dialogue = teloxide::dispatching::dialogue::Dialogue<State, Storage>;

type OptionsCheckboxList = CheckboxList<Options, OptionsCLS>;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
enum State {
    #[default]
    Idle,
    EditingWidget(Widget),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Widget {
    pub options: OptionsCheckboxList,
}

#[derive(Debug, Display, Clone, Deserialize, Serialize)]
enum Options {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct OptionsCLS;

impl CheckboxListSettings for OptionsCLS {
    fn prefix() -> &'static str {
        "o_"
    }

    fn size() -> (u8, u8) {
        // Try to change it to (1, 3)
        (3, 1)
    }
}

impl UserDefinedWidget for Widget {
    type Err = Error;
    type Bot = Bot;
    type Dialogue = Dialogue;

    fn schema() -> UpdateHandler {
        dptree::entry().branch(OptionsCheckboxList::schema::<Self>())
    }

    fn inline_keyboard_markup(&self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(self.options.keyboard())
    }

    fn update_state(
        self,
        dialogue: &Self::Dialogue,
    ) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async move {
            dialogue.update(State::EditingWidget(self)).await?;

            Ok(())
        }
    }
}

impl WidgetContainer<OptionsCheckboxList> for Widget {
    fn get_widget(&mut self) -> &mut OptionsCheckboxList {
        &mut self.options
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Example \"checkbox_list\" started..");

    let state_storage = InMemStorage::<State>::new();

    Dispatcher::builder(Bot::from_env(), schema())
        .dependencies(dptree::deps![state_storage])
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler {
    dptree::entry()
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, Storage, State>()
                .endpoint(send_widget),
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, Storage, State>()
                .branch(dptree::case![State::EditingWidget(_w)].branch(Widget::schema())),
        )
}

async fn send_widget(bot: Bot, dialogue: Dialogue, message: Message) -> HandlerResult {
    let options = CheckboxList::new([Options::A, Options::B, Options::C]);

    let widget = Widget { options };

    bot.send_message(message.chat.id, "Choose options:")
        .reply_markup(widget.inline_keyboard_markup())
        .await?;

    dialogue.update(State::EditingWidget(widget)).await?;

    Ok(())
}
