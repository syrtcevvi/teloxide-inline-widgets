//! This example demonstrates how to use the `Calendar` widget.
use serde::{Deserialize, Serialize};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use teloxide_inline_widgets::{prelude::*, types::WidgetStyles, Calendar};

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
    ChoosingDate(ChooseDateWidget),
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, InlineWidget)]
#[inline_widget(err_ty = Error, bot_ty = Bot, dialogue_ty = Dialogue)]
#[inline_widget(state = State::ChoosingDate)]
struct ChooseDateWidget {
    #[calendar(day_click = show_clicked_day, weekday_click = show_clicked_weekday)]
    pub calendar: Calendar,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Example \"calendar\" started..");

    let state_storage = InMemStorage::<State>::new();

    Dispatcher::builder(Bot::from_env(), schema())
        .dependencies(dptree::deps![state_storage, WidgetStyles::default()])
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
                .branch(dptree::case![State::ChoosingDate(_w)].branch(ChooseDateWidget::schema())),
        )
}

async fn send_widget(
    bot: Bot,
    dialogue: Dialogue,
    message: Message,
    widget_styles: WidgetStyles,
) -> HandlerResult {
    let widget = ChooseDateWidget::default();

    bot.send_message(message.chat.id, "Choose a date:")
        .reply_markup(widget.inline_keyboard_markup(&widget_styles))
        .await?;

    dialogue.update(State::ChoosingDate(widget)).await?;

    Ok(())
}

async fn show_clicked_day(bot: Bot, cq: CallbackQuery, clicked_day: NaiveDate) -> HandlerResult {
    bot.answer_callback_query(cq.id).await?;

    bot.send_message(
        cq.message.unwrap().chat.id,
        format!("You've clicked: {}", clicked_day.format("%Y-%m-%d")),
    )
    .await?;

    Ok(())
}

async fn show_clicked_weekday(
    bot: Bot,
    cq: CallbackQuery,
    clicked_weekday: Weekday,
) -> HandlerResult {
    bot.answer_callback_query(cq.id).await?;

    bot.send_message(cq.message.unwrap().chat.id, format!("You've clicked: {clicked_weekday}"))
        .await?;

    Ok(())
}
