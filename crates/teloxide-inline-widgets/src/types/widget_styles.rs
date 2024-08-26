use std::sync::Arc;

use crate::types::{CheckboxListStyle, RadioListStyle};

#[derive(Debug, Clone, Default)]
pub struct WidgetStyles {
    pub radio_list_style: Arc<RadioListStyle>,
    pub checkbox_list_style: Arc<CheckboxListStyle>,
}
