use darling::FromField;

/// Arguments for the `#[radio_list]` field attribute
#[derive(Debug, FromField)]
#[darling(attributes(radio_list))]
pub struct RadioListArgs {
    pub prefix: String,
    // TODO opt?
    pub rows: u8,
    pub columns: u8,
}
