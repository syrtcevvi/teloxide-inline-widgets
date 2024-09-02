use std::sync::Arc;

use crate::types::{CalendarStyle, CheckboxListStyle, CommonStyle, RadioListStyle};

#[derive(Debug, Clone, Default)]
pub struct WidgetStyles {
    pub radio_list_style: Arc<RadioListStyle>,
    pub checkbox_list_style: Arc<CheckboxListStyle>,
    pub calendar_style: Arc<CalendarStyle>,
    pub common_style: Arc<CommonStyle>,
}
