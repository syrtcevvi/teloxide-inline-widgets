/*
    This example demonstrates how to use multiple widgets (`RadioList` and `CheckboxList`) within
    the user-define widget.
*/
use std::future::Future;

use derive_more::Display;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::InlineKeyboardMarkup};
use teloxide_inline_widgets::{
    layout::{Layout, LayoutOrientation},
    prelude::*,
};

type Bot = teloxide::Bot;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type HandlerResult = Result<(), Error>;
type UpdateHandler = teloxide::dispatching::UpdateHandler<Error>;
type Storage = InMemStorage<State>;
type Dialogue = teloxide::dispatching::dialogue::Dialogue<State, Storage>;

type ShapesRadioList = RadioList<Shape, ShapesRLS>;
type OptionsCheckboxList = CheckboxList<Options, OptionsCLS>;

#[derive(Debug, Default, Clone)]
enum State {
    #[default]
    Idle,
    EditingComplexWidget(ComplexWidget),
}

#[derive(Debug, Clone)]
struct ComplexWidget {
    pub shapes: ShapesRadioList,
    pub options: OptionsCheckboxList,
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
enum Options {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy)]
struct ShapesRLS;

impl RadioListSettings for ShapesRLS {
    fn prefix() -> &'static str {
        "s_"
    }

    fn size() -> (u8, u8) {
        (4, 1)
    }
}
#[derive(Debug, Clone, Copy)]
struct OptionsCLS;

impl CheckboxListSettings for OptionsCLS {
    fn prefix() -> &'static str {
        "o_"
    }

    fn size() -> (u8, u8) {
        (3, 1)
    }
}

impl UserDefinedWidget for ComplexWidget {
    type Err = Error;
    type Bot = Bot;
    type Dialogue = Dialogue;

    fn schema() -> UpdateHandler {
        dptree::entry()
            .branch(ShapesRadioList::schema::<Self>())
            .branch(OptionsCheckboxList::schema::<Self>())
    }

    fn inline_keyboard_markup(&self) -> InlineKeyboardMarkup {
        Layout {
            widgets: &[&self.shapes, &self.options],
            orientation: LayoutOrientation::Horizontal,
        }
        .into()
    }

    fn update_state(
        self,
        dialogue: &Self::Dialogue,
    ) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async move {
            dialogue.update(State::EditingComplexWidget(self)).await?;

            Ok(())
        }
    }
}

impl WidgetContainer<ShapesRadioList> for ComplexWidget {
    fn get_widget(&mut self) -> &mut ShapesRadioList {
        &mut self.shapes
    }
}

impl WidgetContainer<OptionsCheckboxList> for ComplexWidget {
    fn get_widget(&mut self) -> &mut OptionsCheckboxList {
        &mut self.options
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Example \"multiple_widgets\" started..");

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
                .endpoint(send_complex_widget),
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, Storage, State>()
                .branch(
                    dptree::case![State::EditingComplexWidget(_w)].branch(ComplexWidget::schema()),
                ),
        )
}

async fn send_complex_widget(bot: Bot, dialogue: Dialogue, message: Message) -> HandlerResult {
    let shapes = RadioList::new([Shape::Square, Shape::Triangle, Shape::Circle], None);

    let options = CheckboxList::new([Options::A, Options::B, Options::C]);

    let complex_widget = ComplexWidget { shapes, options };

    bot.send_message(message.chat.id, "Choose shape and options:")
        .reply_markup(complex_widget.inline_keyboard_markup())
        .await?;

    dialogue
        .update(State::EditingComplexWidget(complex_widget))
        .await?;

    Ok(())
}
