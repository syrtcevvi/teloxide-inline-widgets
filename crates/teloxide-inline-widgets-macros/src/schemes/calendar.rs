use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Path};

use crate::attribute_parameters::CalendarParameters;

pub struct CalendarSchemaTypes {
    pub widget_ty: Ident,
    pub bot_ty: Path,
    pub dialogue_ty: Path,
}

/// Handler schema for the [`Calendar`] widget
pub fn calendar_schema(
    CalendarSchemaTypes { widget_ty, bot_ty, dialogue_ty }: &CalendarSchemaTypes,
    CalendarParameters {
        day_click_handler,
        day_prefix,
        weekday_prefix,
        weekday_click_handler,
        prev_month,
        next_month,
        prev_year,
        next_year,
        noop_data,
    }: &CalendarParameters,
) -> TokenStream2 {
    let weekday_click_handler = if let Some(weekday_click_handler) = weekday_click_handler {
        quote! {
            .branch(
                dptree::filter_map(|CallbackQueryData(cq_data): CallbackQueryData| {
                    Weekday::try_from(
                        cq_data.strip_prefix(#weekday_prefix)?.parse::<u8>().ok()?
                    ).ok()
                })
                .endpoint(#weekday_click_handler)
            )
        }
    } else {
        quote! {}
    };

    quote! {
    .branch(
        dptree::entry()
        .filter_map(|cq: CallbackQuery| cq.data.map(|data| CallbackQueryData(data)))
        .branch(
            dptree::filter_map(|CallbackQueryData(cq_data): CallbackQueryData| {
                NaiveDate::parse_from_str(
                    cq_data.strip_prefix(#day_prefix)?,
                    "%Y/%m/%d",
                )
                .ok()
            })
            .endpoint(#day_click_handler)
        )
        #weekday_click_handler
        .filter_map(|cq: CallbackQuery| cq.message.map(|msg| (msg.chat.id, msg.id, cq.id)))
        .endpoint(
            |
                mut widget: #widget_ty,
                bot: #bot_ty,
                dialogue: #dialogue_ty,
                (chat_id, message_id, cq_id): (ChatId, MessageId, String),
                CallbackQueryData(cq_data): CallbackQueryData,
                widget_styles: WidgetStyles
            | async move {
                bot.answer_callback_query(cq_id).await?;

                if cq_data == #noop_data {
                    // TODO possibly custom notification here?
                    return Ok(())
                }

                let calendar = widget.get_widget();
                match cq_data.as_str() {
                    #prev_year => calendar.set_previous_year(),
                    #next_year => calendar.set_next_year(),
                    #prev_month => calendar.set_previous_month(),
                    #next_month => calendar.set_next_month(),
                    _ => {
                        log::warn!("`Calendar` widget received strange `CallbackQuery::data`: \"{cq_data}\"");
                    }
                }
                widget.redraw(&bot, chat_id, message_id, &widget_styles).await?;
                widget.update_state(&dialogue).await?;

                Ok(())
            },
        ),
    )
    }
}
