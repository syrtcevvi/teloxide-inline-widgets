use std::iter::repeat;

use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::{
    traits::GetSize,
    types::{Size, WidgetStyles},
};

// TODO put behind the `calendar` feature-flag

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
            self.month -= 1;
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
                    calendar_style.previous_month_icon.clone(),
                    parameters.previous_month_data,
                ),
                InlineKeyboardButton::callback(
                    calendar_style.months[(self.month - 1) as usize].clone(),
                    parameters.noop_data,
                ),
                InlineKeyboardButton::callback(
                    calendar_style.next_month_icon.clone(),
                    parameters.next_month_data,
                ),
                InlineKeyboardButton::callback(
                    calendar_style.previous_year_icon.clone(),
                    parameters.previous_year_data,
                ),
                InlineKeyboardButton::callback(self.year.to_string(), parameters.noop_data),
                InlineKeyboardButton::callback(
                    calendar_style.next_year_icon.clone(),
                    parameters.next_year_data,
                ),
            ],
            calendar_style
                .days_of_the_week
                .iter()
                .enumerate()
                .map(|(i, weekday)| {
                    InlineKeyboardButton::callback(
                        weekday.clone(),
                        format!("{}{}", parameters.weekday_prefix, i),
                    )
                })
                .collect(),
        ]);
        let month_first_day = NaiveDate::from_ymd_opt(self.year as i32, self.month, 1).unwrap();
        let month_last_day =
            NaiveDate::from_ymd_opt(self.year as i32, self.month, self.days_in_selected_month())
                .unwrap();

        let mut day_buttons: Vec<InlineKeyboardButton> = Vec::with_capacity(31);
        let common_style = &styles.common_style;
        let top_empty_cells_quantity = month_first_day.weekday().num_days_from_monday();
        day_buttons.extend(
            repeat(InlineKeyboardButton::callback(
                common_style.empty_cell_icon.clone(),
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
                common_style.empty_cell_icon.clone(),
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
    pub day_prefix: &'static str,
    pub weekday_prefix: &'static str,
    pub previous_year_data: &'static str,
    pub next_year_data: &'static str,
    pub previous_month_data: &'static str,
    pub next_month_data: &'static str,
    pub noop_data: &'static str,
}
