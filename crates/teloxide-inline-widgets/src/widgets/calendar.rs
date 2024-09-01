use std::{collections::HashMap, iter::repeat};

use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use teloxide::{
    dispatching::UpdateHandler,
    dptree,
    prelude::Requester,
    types::{CallbackQuery, ChatId, InlineKeyboardButton, InlineKeyboardMarkup, MessageId},
};

use crate::{
    traits::{GetSize, InlineWidget, WidgetContainer},
    types::{Size, WidgetStyles},
};

// TODO put behind the `calendar` feature-flag

#[derive(Debug, Clone, Copy)]
pub enum CalendarAction {
    ChoosePreviousYear,
    ChooseNextYear,
    ChoosePreviousMonth,
    ChooseNextMonth,
}

/// Calendar widget
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Calendar {
    /// Currently selected year
    year: u32,
    /// Currently selected month, 1-based
    month: u32,
}

impl Default for Calendar {
    fn default() -> Self {
        let now = Local::now();
        Self { year: now.year() as u32, month: now.month() }
    }
}

impl Calendar {
    /// Creates the [`Calendar`] widget with the current `month` set
    ///
    /// Uses the [`chrono::Local`] to get the current month
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates the [`Calendar`] widget with the selected `year` and `month`
    pub fn with_ym(year: u32, month: u32) -> Self {
        Self { year, month }
    }

    /// Points the [`Calendar`] to the current month and the current year
    pub fn set_current_month(&mut self) {
        let now = Local::now();

        self.year = now.year() as u32;
        self.month = now.month();
    }

    /// Points the [`Calendar`] to the previous year
    pub fn set_previous_year(&mut self) {
        self.year -= 1;
    }

    /// Points the [`Calendar`] to the next year
    pub fn set_next_year(&mut self) {
        self.year += 1;
    }

    /// Points the [`Calendar`] to the previous month
    pub fn set_previous_month(&mut self) {
        if self.month == 1 {
            self.month = 12;
            self.year -= 1;
        } else {
            self.month = self.month - 1;
        }
    }

    /// Points the [`Calendar`] to the next month
    pub fn set_next_month(&mut self) {
        if self.month == 12 {
            self.month = 1;
            self.year += 1;
        } else {
            self.month += 1;
        }
    }

    /// Returns the number of days in the selected month
    pub fn days_in_selected_month(&self) -> u32 {
        let (year, month) = (self.year, self.month);
        NaiveDate::from_ymd_opt(
            match month {
                12 => year + 1,
                _ => year,
            } as i32,
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        )
        .unwrap()
        .signed_duration_since(NaiveDate::from_ymd_opt(year as i32, month, 1).unwrap())
        .num_days() as u32
    }

    /// Returns the first day in the selected month
    pub fn first_day_in_month(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year as i32, self.month, 1).unwrap()
    }

    /// Returns the last day in the selected month
    pub fn last_day_in_month(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year as i32, self.month, self.days_in_selected_month())
            .unwrap()
    }

    pub fn schema<W>(parameters: &'static CalendarSchemaParameters) -> UpdateHandler<W::Err>
    where
        W: 'static + Clone + Send + Sync + InlineWidget + WidgetContainer<Self>,
        W::Bot: 'static + Clone + Send + Sync,
        W::Dialogue: 'static + Clone + Send + Sync,
    {
        let cq_data_to_calendar_action: HashMap<&'static str, CalendarAction> =
            HashMap::from_iter(vec![
                (parameters.previous_year_data, CalendarAction::ChoosePreviousYear),
                (parameters.next_year_data, CalendarAction::ChooseNextYear),
                (parameters.previous_month_data, CalendarAction::ChoosePreviousMonth),
                (parameters.next_month_data, CalendarAction::ChooseNextMonth),
            ]);

        dptree::entry()
            .branch(
                dptree::filter_map(|cq: CallbackQuery| {
                    NaiveDate::parse_from_str(
                        cq.data?.strip_prefix(parameters.day_prefix)?,
                        "%Y/%m/%d",
                    )
                    .ok()
                }), // TODO callback for selected day here
            )
            // TODO weekdays
            // .branch(
            //     dptree::filter_map(|cq: CallbackQuery| )
            // )
            .filter_map(move |cq: CallbackQuery| {
                cq_data_to_calendar_action.get(cq.data?.as_str()).cloned()
            })
            .filter_map(|cq: CallbackQuery| cq.message.map(|msg| (msg.chat.id, msg.id, cq.id)))
            .branch(
                // FIXME: unnecessary in the future
                dptree::filter(|calendar_action: CalendarAction| {
                    use CalendarAction::*;
                    log::info!("{:#?}", calendar_action);
                    match calendar_action {
                        ChoosePreviousYear | ChooseNextYear | ChoosePreviousMonth
                        | ChooseNextMonth => true,
                        _ => false,
                    }
                })
                .endpoint(
                    |bot: W::Bot,
                     dialogue: W::Dialogue,
                     mut widget: W,
                     calendar_action: CalendarAction,
                     (chat_id, message_id, cq_id): (ChatId, MessageId, String),
                     widget_styles: WidgetStyles| async move {
                        bot.answer_callback_query(cq_id).await?;

                        let calendar = widget.get_widget();
                        use CalendarAction::*;
                        match calendar_action {
                            ChoosePreviousYear => calendar.set_previous_year(),
                            ChooseNextYear => calendar.set_next_year(),
                            ChoosePreviousMonth => calendar.set_previous_month(),
                            ChooseNextMonth => calendar.set_next_month(),
                        }
                        widget.redraw(&bot, chat_id, message_id, &widget_styles).await?;
                        widget.update_state(&dialogue).await?;

                        Ok(())
                    },
                ),
            )
    }

    pub fn inline_keyboard_markup(
        &self,
        parameters: &CalendarSchemaParameters,
        styles: &WidgetStyles,
    ) -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::with_capacity(8);

        let calendar_style = &styles.calendar_style;
        // Calendar header
        keyboard.extend([
            vec![
                InlineKeyboardButton::callback(
                    calendar_style.previous_month_icon.to_owned(),
                    parameters.previous_month_data,
                ),
                InlineKeyboardButton::callback(
                    calendar_style.months[(self.month - 1) as usize].to_owned(),
                    parameters.noop_data,
                ),
                InlineKeyboardButton::callback(
                    calendar_style.next_month_icon.to_owned(),
                    parameters.next_month_data,
                ),
                InlineKeyboardButton::callback(
                    calendar_style.previous_year_icon.to_owned(),
                    parameters.previous_year_data,
                ),
                InlineKeyboardButton::callback(self.year.to_string(), parameters.noop_data),
                InlineKeyboardButton::callback(
                    calendar_style.next_year_icon.to_owned(),
                    parameters.next_year_data,
                ),
            ],
            calendar_style
                .days_of_the_week
                .iter()
                .enumerate()
                // TODO weekdays
                .map(|(i, weekday)| {
                    InlineKeyboardButton::callback(
                        weekday.to_owned(),
                        format!("{}{}", parameters.weekday_prefix, i),
                    )
                })
                .collect(),
        ]);
        // TODO в зависимости от языкового кода...
        let month_first_day = NaiveDate::from_ymd_opt(self.year as i32, self.month, 1).unwrap();
        let month_last_day = NaiveDate::from_ymd_opt(
            self.year as i32,
            self.month,
            self.days_in_selected_month() as u32,
        )
        .unwrap();

        let mut day_buttons: Vec<InlineKeyboardButton> = Vec::with_capacity(31);
        let common_style = &styles.common_style;
        let top_empty_cells_quantity = month_first_day.weekday().num_days_from_monday();
        day_buttons.extend(
            repeat(InlineKeyboardButton::callback(
                common_style.empty_cell_icon.to_owned(),
                parameters.noop_data,
            ))
            .take(top_empty_cells_quantity as usize),
        );
        day_buttons.extend((1..=self.days_in_selected_month()).map(|day| {
            InlineKeyboardButton::callback(
                day.to_string(),
                format!("{}{}/{}/{}", parameters.day_prefix, self.year, self.month, day),
            )
        }));
        let bottom_empty_cells_quantity: u32 =
            7u32 - month_last_day.weekday().num_days_from_monday() - 1;
        day_buttons.extend(
            repeat(InlineKeyboardButton::callback(
                common_style.empty_cell_icon.to_owned(),
                parameters.noop_data,
            ))
            .take(bottom_empty_cells_quantity as usize),
        );

        for row in day_buttons.chunks(7) {
            keyboard.push(row.to_vec())
        }

        InlineKeyboardMarkup::new(keyboard)
    }
}

impl GetSize for Calendar {
    fn size(&self) -> Size {
        Size { rows: 8, columns: 7 }
    }
}

pub struct CalendarSchemaParameters {
    pub previous_year_data: &'static str,
    pub next_year_data: &'static str,
    pub previous_month_data: &'static str,
    pub next_month_data: &'static str,
    pub noop_data: &'static str,
    pub day_prefix: &'static str,
    pub weekday_prefix: &'static str,
}
