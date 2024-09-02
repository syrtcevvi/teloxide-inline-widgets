use darling::FromField;

/// Arguments for the `#[checkbox_list]` field attribute
#[derive(Debug, FromField)]
#[darling(attributes(checkbox_list))]
pub struct CheckboxListParameters {
    /// CallbackQuery data prefix to be sent with the index of the clicked item
    pub prefix: String,
    /// CallbackQuery data for empty cells
    pub noop_data: Option<String>,
}
