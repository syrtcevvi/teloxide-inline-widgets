/*
    This example demonstrates how to use the `CheckboxList` widget.
*/
use derive_more::Display;
use serde::{Deserialize, Serialize};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use teloxide_inline_widgets::{prelude::*, CheckboxList};

type Bot = teloxide::Bot;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type HandlerResult = Result<(), Error>;
type UpdateHandler = teloxide::dispatching::UpdateHandler<Error>;
type Storage = InMemStorage<State>;
type Dialogue = teloxide::dispatching::dialogue::Dialogue<State, Storage>;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
enum State {
    #[default]
    Idle,
    ChoosingVariants(ChooseVariantsWidget),
}

#[derive(Debug, Clone, Deserialize, Serialize, InlineWidget)]
#[inline_widget(err_ty = Error, bot_ty = Bot, dialogue_ty = Dialogue)]
#[inline_widget(state = State::ChoosingVariants)]
struct ChooseVariantsWidget {
    #[checkbox_list(prefix = "v_", rows = 3, columns = 1)]
    pub variants: CheckboxList<Variant>,
}

#[derive(Debug, Display, Clone, Deserialize, Serialize)]
enum Variant {
    A,
    B,
    C,
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
                .branch(
                    dptree::case![State::ChoosingVariants(_w)]
                        .branch(ChooseVariantsWidget::schema()),
                ),
        )
}

async fn send_widget(bot: Bot, dialogue: Dialogue, message: Message) -> HandlerResult {
    let options = CheckboxList::from(vec![Variant::A, Variant::B, Variant::C]);

    let widget = ChooseVariantsWidget { variants: options };

    bot.send_message(message.chat.id, "Choose variants:")
        .reply_markup(widget.inline_keyboard_markup())
        .await?;

    dialogue.update(State::ChoosingVariants(widget)).await?;

    Ok(())
}
