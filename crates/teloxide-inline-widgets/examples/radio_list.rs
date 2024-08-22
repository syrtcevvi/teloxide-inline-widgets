/*
    This example demonstrates how to use the `RadioList` widget.
*/
use derive_more::Display;
use serde::{Deserialize, Serialize};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use teloxide_inline_widgets::{prelude::*, RadioList};

type Bot = teloxide::Bot;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type HandlerResult = Result<(), Error>;
type UpdateHandler = teloxide::dispatching::UpdateHandler<Error>;
type Storage = InMemStorage<State>;
type Dialogue = teloxide::dispatching::dialogue::Dialogue<State, Storage>;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
enum State {
    #[default]
    Idle,
    ChoosingFruit(ChooseFruitWidget),
}

#[derive(Debug, Clone, Deserialize, Serialize, InlineWidget)]
#[inline_widget(err_ty = Error, bot_ty = Bot, dialogue_ty = Dialogue)]
#[inline_widget(state = State::ChoosingFruit)]
struct ChooseFruitWidget {
    #[component(prefix = "f_", rows = 1, columns = 3)]
    pub fruits: RadioList<Fruit>,
}

#[derive(Debug, Display, Clone, Deserialize, Serialize)]
#[display(fmt = "{name} ${cost}")]
struct Fruit {
    pub name: String,
    pub cost: u32,
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
                .branch(
                    dptree::case![State::ChoosingFruit(_w)].branch(ChooseFruitWidget::schema()),
                ),
        )
}

async fn send_widget(bot: Bot, dialogue: Dialogue, message: Message) -> HandlerResult {
    let fruits = RadioList::new(
        vec![Fruit { name: "Apple".into(), cost: 42 }, Fruit { name: "Pear".into(), cost: 13 }],
        None,
    );

    let widget = ChooseFruitWidget { fruits };

    bot.send_message(message.chat.id, "Choose a fruit:")
        .reply_markup(widget.inline_keyboard_markup())
        .await?;

    dialogue.update(State::ChoosingFruit(widget)).await?;

    Ok(())
}
