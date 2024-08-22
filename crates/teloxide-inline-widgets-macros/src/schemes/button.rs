use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Path;

/// Handler schema for the [`Button`] widget
pub fn button_schema(cq_data: String, click_handler: Path) -> TokenStream2 {
    quote! {
        .branch(
            dptree::entry()
            .filter(move |cq: CallbackQuery| {
                cq.data.unwrap_or("".to_owned()) == #cq_data
            })
            .endpoint(#click_handler)
        )
    }
}
