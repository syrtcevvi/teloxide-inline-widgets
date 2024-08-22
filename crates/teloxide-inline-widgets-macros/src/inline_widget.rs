use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Ident, Path, Type, TypePath};

const RADIO_LIST_TYPE: &str = "RadioList";
const CHECKBOX_LIST_TYPE: &str = "CheckboxList";

/// Arguments for the top-level `#[inline_widget]` struct attribute
#[derive(Debug, FromField)]
#[darling(attributes(component))]
struct InlineWidgetComponentArgs {
    ident: Option<Ident>,
    ty: Type,
    prefix: String,
    rows: u8,
    columns: u8,
}

/// Arguments for the `#[component]` field attribute
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(inline_widget))]
struct InlineWidgetArgs {
    /// Error type
    err_ty: Path,
    /// Bot type
    bot_ty: Path,
    /// Dialogue type
    dialogue_ty: Path,
    /// Variant for the widget state
    state: Path,
    ///
    layout_orientation: Option<Path>,
}

pub(crate) fn inline_widget_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_ident = input.ident.clone();

    if let Data::Struct(DataStruct { fields, .. }) = &input.data {
        let InlineWidgetArgs { err_ty, bot_ty, dialogue_ty, state, layout_orientation } =
            match InlineWidgetArgs::from_derive_input(&input) {
                Ok(v) => v,
                Err(e) => return TokenStream::from(e.write_errors()),
            };

        let mut widget_container_impls = quote! {};
        let mut schema = quote! {
            dptree::entry()
        };
        let mut markups = vec![];

        for field in fields {
            let InlineWidgetComponentArgs { ident, ty: component_ty, prefix, rows, columns } =
                match InlineWidgetComponentArgs::from_field(field) {
                    Ok(v) => v,
                    Err(e) => return TokenStream::from(e.write_errors()),
                };
            let field_ident = ident.unwrap();

            let type_name = get_type_name(&component_ty);

            widget_container_impls.extend(quote! {
                impl WidgetContainer<#component_ty> for #struct_ident {
                    fn get_widget(&mut self) -> &mut #component_ty {
                        &mut self.#field_ident
                    }
                }
            });

            schema.extend(quote! {
                .branch(<#component_ty>::schema::<#struct_ident>(#prefix))
            });

            match type_name.as_str() {
                RADIO_LIST_TYPE | CHECKBOX_LIST_TYPE => {
                    markups.push(quote! {
                        self.#field_ident.inline_keyboard_markup(#prefix, (#rows, #columns))
                    });
                }
                _ => unimplemented!(),
            };
        }

        let layout_orientation = if let Some(layout_orientation) = layout_orientation {
            quote! {
                #layout_orientation
            }
        } else {
            quote! {
                LayoutOrientation::Vertical
            }
        };

        quote! {
            #widget_container_impls

            impl InlineWidget for #struct_ident {
                type Err = #err_ty;
                type Bot = #bot_ty;
                type Dialogue = #dialogue_ty;

                fn schema() -> teloxide::dispatching::UpdateHandler<Self::Err> {
                    #schema
                }

                fn inline_keyboard_markup(&self) -> teloxide::types::InlineKeyboardMarkup {
                    Layout {
                        markups: vec![#(#markups),*],
                        orientation: #layout_orientation
                    }.into()
                }

                async fn update_state(
                    self,
                    dialogue: &Self::Dialogue
                ) -> Result<(), Self::Err> {
                    dialogue.update(
                        #state(self)
                    ).await?;

                    Ok(())
                }
            }
        }
        .into()
    } else {
        panic!("Deriving InlineWidget is only supported for structs with named fields");
    }
}

fn get_type_name(ty: &Type) -> String {
    match ty {
        syn::Type::Path(TypePath { path, .. }) => {
            path.segments.last().unwrap().ident.to_string()
        }
        _ => panic!("Unable to get the type name for: {ty:#?}"),
    }
}
