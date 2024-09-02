//! This example demonstrates how to alter the widget styles.
use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use derive_more::Display;
use serde::{Deserialize, Serialize};
use teloxide::{dispatching::dialogue::InMemStorage, macros::BotCommands, prelude::*};
use teloxide_inline_widgets::{
    prelude::*,
    types::{RadioListStyle, WidgetStyles},
    RadioList,
};

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

#[derive(Debug, Clone, BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    Icon(String),
}

#[derive(Debug, Clone, Deserialize, Serialize, InlineWidget)]
#[inline_widget(err_ty = Error, bot_ty = Bot, dialogue_ty = Dialogue)]
#[inline_widget(state = State::ChoosingFruit)]
struct ChooseFruitWidget {
    #[radio_list(prefix = "f_")]
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

    log::info!("Example \"styles\" started..");

    let state_storage = InMemStorage::<State>::new();
    let widget_styles = WidgetStyles::default();

    // We need some storage for changing `RadioList`::active_icon value.
    // Supposed that the style can be changed per user, so there is no ability for
    // changing widget styles globally during the run
    let icon = Arc::new(Mutex::new(widget_styles.radio_list_style.active_icon.to_string()));

    Dispatcher::builder(Bot::from_env(), schema())
        .dependencies(dptree::deps![state_storage, widget_styles, icon])
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler {
    dptree::entry()
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, Storage, State>()
                .branch(dptree::entry().filter_command::<Command>().branch(
                    dptree::case![Command::Icon(_icon)].inspect(
                        |icon: String, icon_storage: Arc<Mutex<String>>| {
                            *icon_storage.lock().unwrap() = icon;
                        },
                    ),
                ))
                .branch(dptree::endpoint(send_widget)),
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, Storage, State>()
                .branch(
                    dptree::case![State::ChoosingFruit(_w)]
                        // Substitues the `WidgetStyles` object with new one in the dependencies
                        .map(alter_radio_list_style)
                        .branch(ChooseFruitWidget::schema()),
                ),
        )
}

fn alter_radio_list_style(
    icon: Arc<Mutex<String>>,
    mut widget_styles: WidgetStyles,
) -> WidgetStyles {
    // Normally style for each user (chat) should arrive from some database or other
    // persistent storage or so
    widget_styles.radio_list_style =
        RadioListStyle::builder().active_icon(Cow::Owned(icon.lock().unwrap().clone())).build();

    widget_styles
}

async fn send_widget(
    bot: Bot,
    dialogue: Dialogue,
    message: Message,
    widget_styles: WidgetStyles,
) -> HandlerResult {
    let fruits = RadioList::new(
        vec![Fruit { name: "Apple".into(), cost: 42 }, Fruit { name: "Pear".into(), cost: 13 }],
        None,
        Size::new(2, 2),
    );

    let widget = ChooseFruitWidget { fruits };

    bot.send_message(message.chat.id, "Choose a fruit:")
        .reply_markup(widget.inline_keyboard_markup(&widget_styles))
        .await?;

    dialogue.update(State::ChoosingFruit(widget)).await?;

    Ok(())
}
