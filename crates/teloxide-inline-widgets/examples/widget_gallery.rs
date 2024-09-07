/*
    This example demonstrates how to use different widget (all-in-one example)
*/
use serde::{Deserialize, Serialize};
use teloxide::{dispatching::dialogue::InMemStorage, macros::BotCommands, prelude::*, types::User};

use teloxide_inline_widgets::{
    prelude::*, types::WidgetStyles, Button, Calendar, CheckboxList, RadioList,
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
    RadioList(RadioListWidget),
    CheckboxList(CheckboxListWidget),
    Calendar(CalendarWidget),
}

#[derive(Debug, Clone, Deserialize, Serialize, InlineWidget)]
#[inline_widget(err_ty = Error, bot_ty = Bot, dialogue_ty = Dialogue)]
#[inline_widget(state = State::RadioList)]
struct RadioListWidget {
    #[radio_list(prefix = "rl_")]
    pub radio_list: RadioList<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize, InlineWidget)]
#[inline_widget(err_ty = Error, bot_ty = Bot, dialogue_ty = Dialogue)]
#[inline_widget(state = State::CheckboxList)]
struct CheckboxListWidget {
    #[checkbox_list(prefix = "cl_")]
    pub checkbox_list: CheckboxList<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize, InlineWidget)]
#[inline_widget(err_ty = Error, bot_ty = Bot, dialogue_ty = Dialogue)]
#[inline_widget(state = State::Calendar)]
struct CalendarWidget {
    #[calendar(day_click = show_clicked_day, weekday_click = show_clicked_weekday)]
    pub calendar: Calendar,
}

#[derive(Debug, Clone, Deserialize, Serialize, InlineWidget)]
#[inline_widget(err_ty = Error, bot_ty = Bot)]
struct ButtonWidget {
    #[button(data = "b", click = say_hello)]
    pub button: Button,
}

#[derive(Debug, Clone, PartialEq, BotCommands)]
#[command(rename_rule = "snake_case")]
enum Command {
    RadioList,
    CheckboxList,
    Button,
    Calendar,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Example \"widget_gallery\" started..");

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
                .filter_command::<Command>()
                .filter_map(|update: Update| update.user().cloned())
                .enter_dialogue::<Message, Storage, State>()
                .endpoint(send_widget),
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, Storage, State>()
                .branch(dptree::case![State::RadioList(_w)].branch(RadioListWidget::schema()))
                .branch(dptree::case![State::CheckboxList(_w)].branch(CheckboxListWidget::schema()))
                .branch(dptree::case![State::Calendar(_w)].branch(CalendarWidget::schema()))
                .branch(ButtonWidget::schema()),
        )
}

async fn send_widget(
    bot: Bot,
    dialogue: Dialogue,
    user: User,
    command: Command,
    widget_styles: WidgetStyles,
) -> HandlerResult {
    match command {
        Command::RadioList => {
            send_radio_list(bot, dialogue, user, widget_styles).await?;
        }
        Command::CheckboxList => {
            send_checkbox_list(bot, dialogue, user, widget_styles).await?;
        }
        Command::Button => {
            send_button(bot, user, widget_styles).await?;
        }
        Command::Calendar => {
            send_calendar(bot, dialogue, user, widget_styles).await?;
        }
    }

    Ok(())
}

async fn send_radio_list(
    bot: Bot,
    dialogue: Dialogue,
    user: User,
    widget_styles: WidgetStyles,
) -> HandlerResult {
    let radio_list = RadioListWidget { radio_list: RadioList::from_iter(1..3) };

    bot.send_message(user.id, "Radio list example:")
        .reply_markup(radio_list.inline_keyboard_markup(&widget_styles))
        .await?;

    dialogue.update(State::RadioList(radio_list)).await?;

    Ok(())
}

async fn send_checkbox_list(
    bot: Bot,
    dialogue: Dialogue,
    user: User,
    widget_styles: WidgetStyles,
) -> HandlerResult {
    let checkbox_list = CheckboxListWidget { checkbox_list: CheckboxList::from_iter(1..5) };

    bot.send_message(user.id, "Checkbox list example:")
        .reply_markup(checkbox_list.inline_keyboard_markup(&widget_styles))
        .await?;

    dialogue.update(State::CheckboxList(checkbox_list)).await?;

    Ok(())
}

async fn send_button(bot: Bot, user: User, widget_styles: WidgetStyles) -> HandlerResult {
    let button = ButtonWidget { button: Button::new("Click me") };

    bot.send_message(user.id, "Button example:")
        .reply_markup(button.inline_keyboard_markup(&widget_styles))
        .await?;

    Ok(())
}

async fn send_calendar(
    bot: Bot,
    dialogue: Dialogue,
    user: User,
    widget_styles: WidgetStyles,
) -> HandlerResult {
    let checkbox_list = CheckboxListWidget { checkbox_list: CheckboxList::from_iter(1..5) };

    bot.send_message(user.id, "Checkbox list example:")
        .reply_markup(checkbox_list.inline_keyboard_markup(&widget_styles))
        .await?;

    dialogue.update(State::CheckboxList(checkbox_list)).await?;

    Ok(())
}

async fn say_hello(bot: Bot, message: Message) -> HandlerResult {
    bot.send_message(message.chat.id, "Hello!").await?;

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
