use darling::FromField;
use syn::Path;

#[derive(Debug, FromField)]
#[darling(attributes(calendar))]
pub struct CalendarParameters {
    /// Handler to be invoked when the day-button is clicked
    pub day_click_handler: Path,
    /// CallbackQuery data to be sent when the `previous year` button is
    /// clicked, `py` by default
    pub prev_year: Option<String>,
    /// CallbackQuery data to be sent when the `next year` button is clicked,
    /// `ny` by default
    pub next_year: Option<String>,
    /// CallbackQuery data to be sent when the `previous month` button is
    /// clicked, `pm` by default
    pub prev_month: Option<String>,
    /// CallbackQuery data to be sent when the `next month` button is clicked,
    /// `nm` by default
    pub next_month: Option<String>,
}
