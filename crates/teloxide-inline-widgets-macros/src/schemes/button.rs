use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::attribute_parameters::ButtonParameters;

/// Handler schema for the [`Button`] widget
pub fn button_schema(ButtonParameters { data, click_handler }: &ButtonParameters) -> TokenStream2 {
    quote! {
        .branch(
            dptree::entry()
            .filter(move |cq: CallbackQuery| {
                cq.data.unwrap_or("".to_owned()) == #data
            })
            .endpoint(#click_handler)
        )
    }
}
