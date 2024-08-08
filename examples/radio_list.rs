/*
    This example demonstrates how to use the `RadioList` widget.
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

type FruitsRadioList = RadioList<Fruit, FruitsRLS>;

#[derive(Debug, Display, Clone, Deserialize, Serialize)]
#[display(fmt = "{name} {cost}")]
struct Fruit {
    pub name: String,
    pub cost: u32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
enum State {
    #[default]
    Idle,
    EditingWidget(Widget),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Widget {
    pub fruits: FruitsRadioList,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
struct FruitsRLS;

impl RadioListSettings for FruitsRLS {
    fn prefix() -> &'static str {
        "f_"
    }

    fn size() -> (u8, u8) {
        (1, 3)
    }
}

impl UserDefinedWidget for Widget {
    type Err = Error;
    type Bot = Bot;
    type Dialogue = Dialogue;

    fn schema() -> UpdateHandler {
        dptree::entry().branch(FruitsRadioList::schema::<Self>())
    }

    fn inline_keyboard_markup(&self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(self.fruits.keyboard())
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

impl WidgetContainer<FruitsRadioList> for Widget {
    fn get_widget(&mut self) -> &mut FruitsRadioList {
        &mut self.fruits
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Example \"radio_list\" started..");

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
    let fruits = FruitsRadioList::new(
        vec![
            Fruit {
                name: "Apple".into(),
                cost: 42,
            },
            Fruit {
                name: "Pear".into(),
                cost: 13,
            },
        ],
        None,
    );

    let widget = Widget { fruits };

    bot.send_message(message.chat.id, "Choose a fruit")
        .reply_markup(widget.inline_keyboard_markup())
        .await?;

    dialogue.update(State::EditingWidget(widget)).await?;

    Ok(())
}
