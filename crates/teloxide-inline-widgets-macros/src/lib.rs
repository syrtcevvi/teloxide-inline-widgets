mod attribute_parameters;
mod constants;
mod inline_widget;
mod schemes;

use proc_macro::TokenStream;

#[proc_macro_derive(
    InlineWidget,
    attributes(inline_widget, radio_list, checkbox_list, button, calendar)
)]
pub fn derive_inline_widget(input: TokenStream) -> TokenStream {
    inline_widget::inline_widget_impl(input)
}
