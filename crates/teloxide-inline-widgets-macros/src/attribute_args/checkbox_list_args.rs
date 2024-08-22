use darling::FromField;

/// Arguments for the `#[checkbox_list]` field attribute
#[derive(Debug, FromField)]
#[darling(attributes(checkbox_list))]
pub struct CheckboxListArgs {
    pub prefix: String,
    // TODO opt?
    pub rows: u8,
    pub columns: u8,
}
