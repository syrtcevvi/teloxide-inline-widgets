use darling::FromField;

/// Arguments for the `#[radio_list]` field attribute
#[derive(Debug, FromField)]
#[darling(attributes(radio_list))]
pub struct RadioListParameters {
    /// CallbackQuery data prefix to be sent with the index of the clicked item
    pub prefix: String,
    /// CallbackQuery data for empty cells
    pub noop_data: Option<String>,
}
