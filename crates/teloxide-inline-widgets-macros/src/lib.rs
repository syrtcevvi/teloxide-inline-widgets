mod inline_widget;

use proc_macro::TokenStream;

#[proc_macro_derive(InlineWidget, attributes(inline_widget, component))]
pub fn derive_inline_widget(input: TokenStream) -> TokenStream {
    inline_widget::inline_widget_impl(input)
}
