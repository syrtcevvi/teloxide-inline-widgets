use darling::FromField;
use syn::Path;

#[derive(Debug, FromField)]
#[darling(attributes(calendar))]
pub struct CalendarParameters {
    /// CallbackQuery data prefix to be sent with the selected day
    pub day_prefix: Option<String>,
    /// Handler to be invoked when the day-button is clicked
    #[darling(rename = "day_click")]
    pub day_click_handler: Path,
    /// CallbackQuery data prefix to be sent with the selected day of the week
    pub weekday_prefix: Option<String>,
    /// Handler to be invoked when the weekday-button is clicked
    #[darling(rename = "weekday_click")]
    pub weekday_click_handler: Option<Path>,
    /// CallbackQuery data to be sent when the `previous year` button is
    /// clicked
    pub prev_year: Option<String>,
    /// CallbackQuery data to be sent when the `next year` button is clicked
    pub next_year: Option<String>,
    /// CallbackQuery data to be sent when the `previous month` button is
    /// clicked
    pub prev_month: Option<String>,
    /// CallbackQuery data to be sent when the `next month` button is clicked
    pub next_month: Option<String>,
    /// CallbackQuery data for empty cells
    pub noop_data: Option<String>,
}
