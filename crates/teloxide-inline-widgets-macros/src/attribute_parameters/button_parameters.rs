use darling::FromField;
use syn::Path;

/// Arguments for the `#[button]` field attribute
#[derive(Debug, FromField)]
#[darling(attributes(button))]
pub struct ButtonParameters {
    /// CallbackQuery data to be sent when the button is clicked
    pub data: String,
    /// Handler to be invoked when the button is clicked
    #[darling(rename = "click")]
    pub click_handler: Path,
}
