//! This example demonstrates how to use the `Button` widget.
use serde::{Deserialize, Serialize};
use teloxide::prelude::*;
use teloxide_inline_widgets::{prelude::*, Button};

type Bot = teloxide::Bot;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type HandlerResult = Result<(), Error>;
type UpdateHandler = teloxide::dispatching::UpdateHandler<Error>;

#[derive(Debug, Clone, Deserialize, Serialize, InlineWidget)]
#[inline_widget(err_ty = Error, bot_ty = Bot, layout_orientation = LayoutOrientation::Horizontal)]
struct ButtonsWidget {
    #[button(data = "a", click = say_a)]
    pub say_a: Button,
    #[button(data = "b", click = say_b)]
    pub say_b: Button,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Example \"buttons\" started..");

    Dispatcher::builder(Bot::from_env(), schema())
        .dependencies(dptree::deps![WidgetStyles::default()])
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler {
    dptree::entry()
        .branch(Update::filter_message().endpoint(send_widget))
        .branch(Update::filter_callback_query().branch(ButtonsWidget::schema()))
}

async fn send_widget(bot: Bot, message: Message, styles: WidgetStyles) -> HandlerResult {
    let widget = ButtonsWidget { say_a: Button::new("Say a"), say_b: Button::new("Say b") };

    bot.send_message(message.chat.id, "Click buttons:")
        .reply_markup(widget.inline_keyboard_markup(&styles))
        .await?;

    Ok(())
}

async fn say_a(bot: Bot, cq: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(cq.id).await?;
    bot.send_message(cq.message.unwrap().chat.id, "Aaaa").await?;
    Ok(())
}

async fn say_b(bot: Bot, cq: CallbackQuery) -> HandlerResult {
    bot.answer_callback_query(cq.id).await?;
    bot.send_message(cq.message.unwrap().chat.id, "Bbbb").await?;
    Ok(())
}
