/*
    This example demonstrates how to use multiple widgets (`RadioList` and `CheckboxList`) within
    the user-define widget.
*/
use derive_more::Display;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use teloxide_inline_widgets::{prelude::*, types::WidgetStyles, Button, CheckboxList, RadioList};

type Bot = teloxide::Bot;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type HandlerResult = Result<(), Error>;
type UpdateHandler = teloxide::dispatching::UpdateHandler<Error>;
type Storage = InMemStorage<State>;
type Dialogue = teloxide::dispatching::dialogue::Dialogue<State, Storage>;

#[derive(Debug, Default, Clone)]
enum State {
    #[default]
    Idle,
    EditingComplexWidget(ComplexWidget),
}

#[derive(Debug, Clone, InlineWidget)]
#[inline_widget(err_ty = Error, bot_ty = Bot, dialogue_ty = Dialogue)]
#[inline_widget(state = State::EditingComplexWidget, layout_orientation =  LayoutOrientation::Horizontal)]
struct ComplexWidget {
    #[radio_list(prefix = "s_", rows = 4, columns = 1)]
    pub shapes: RadioList<Shape>,
    #[checkbox_list(prefix = "o_", rows = 3, columns = 1)]
    pub options: CheckboxList<Variant>,
    #[button(data = "sb", click = process_save)]
    pub save_button: Button,
}

#[derive(Debug, Display, Clone)]
enum Shape {
    #[display(fmt = "square")]
    Square,
    #[display(fmt = "triangle")]
    Triangle,
    #[display(fmt = "circle")]
    Circle,
}

#[derive(Debug, Display, Clone)]
enum Variant {
    A,
    B,
    C,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Example \"multiple_widgets\" started..");

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
                .branch(dptree::case![State::Idle].endpoint(send_complex_widget)),
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, Storage, State>()
                .branch(
                    dptree::case![State::EditingComplexWidget(_w)].branch(ComplexWidget::schema()),
                ),
        )
}

async fn send_complex_widget(
    bot: Bot,
    dialogue: Dialogue,
    message: Message,
    widget_styles: WidgetStyles,
) -> HandlerResult {
    let shapes = RadioList::new([Shape::Square, Shape::Triangle, Shape::Circle], None);

    let options =
        CheckboxList::new([(false, Variant::A), (false, Variant::B), (false, Variant::C)]);

    let complex_widget = ComplexWidget { shapes, options, save_button: Button::new("Save") };

    bot.send_message(message.chat.id, "Choose shape and options:")
        .reply_markup(complex_widget.inline_keyboard_markup(&widget_styles))
        .await?;

    dialogue.update(State::EditingComplexWidget(complex_widget)).await?;

    Ok(())
}

async fn process_save(bot: Bot, dialogue: Dialogue, cq: CallbackQuery) -> HandlerResult {
    let message = cq.message.unwrap();

    bot.answer_callback_query(cq.id).await?;

    bot.delete_message(message.chat.id, message.id).await?;

    bot.send_message(message.chat.id, "Your settings successfully saved!").await?;

    dialogue.update(State::Idle).await?;

    Ok(())
}
