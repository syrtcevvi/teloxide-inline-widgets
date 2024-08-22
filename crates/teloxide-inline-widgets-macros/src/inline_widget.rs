use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Path, Type, TypePath};

use crate::{
    attribute_args::{ButtonArgs, CheckboxListArgs, RadioListArgs},
    schemes::button_schema,
};

const RADIO_LIST_TYPE: &str = "RadioList";
const CHECKBOX_LIST_TYPE: &str = "CheckboxList";
const BUTTON: &str = "Button";

/// Arguments for the top-level `#[inline_widget]` struct attribute
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
    /// Layout orientation kind
    layout_orientation: Option<Path>,
}

#[derive(Default)]
struct InlineWidgetBoilerplate {
    pub schema: TokenStream2,
    pub inline_keyboard_markups: Vec<TokenStream2>,
}

pub(crate) fn inline_widget_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_ident = &input.ident;

    if let Data::Struct(DataStruct { fields, .. }) = &input.data {
        let InlineWidgetArgs { err_ty, bot_ty, dialogue_ty, state, layout_orientation } =
            match InlineWidgetArgs::from_derive_input(&input) {
                Ok(v) => v,
                Err(e) => return TokenStream::from(e.write_errors()),
            };

        let mut inline_widget_boilerplate = InlineWidgetBoilerplate::default();

        let mut widget_container_impls = quote! {};
        let mut schema = quote! {
            dptree::entry()
        };
        let mut markups = vec![];

        for field in fields {
            let field_ident =
                field.ident.as_ref().expect("The user-defined widget field has to be named");
            let field_type = &field.ty;
            let field_type_name = get_type_name(&field.ty);

            widget_container_impls.extend(quote! {
                impl WidgetContainer<#field_type> for #struct_ident {
                    fn get_widget(&mut self) -> &mut #field_type {
                        &mut self.#field_ident
                    }
                }
            });

            match field_type_name.as_str() {
                RADIO_LIST_TYPE => {
                    let RadioListArgs { prefix, rows, columns } =
                        match RadioListArgs::from_field(field) {
                            Ok(args) => args,
                            Err(err) => return TokenStream::from(err.write_errors()),
                        };
                    schema.extend(quote! {
                        .branch(<#field_type>::schema::<#struct_ident>(#prefix))
                    });
                    markups.push(quote! {
                        self.#field_ident.inline_keyboard_markup(#prefix, (#rows, #columns))
                    });
                }
                CHECKBOX_LIST_TYPE => {
                    let CheckboxListArgs { prefix, rows, columns } =
                        match CheckboxListArgs::from_field(field) {
                            Ok(args) => args,
                            Err(err) => return TokenStream::from(err.write_errors()),
                        };
                    schema.extend(quote! {
                        .branch(<#field_type>::schema::<#struct_ident>(#prefix))
                    });
                    markups.push(quote! {
                        self.#field_ident.inline_keyboard_markup(#prefix, (#rows, #columns))
                    });
                }
                BUTTON => {
                    let ButtonArgs { data, click_handler } = match ButtonArgs::from_field(field) {
                        Ok(args) => args,
                        Err(err) => return TokenStream::from(err.write_errors()),
                    };

                    markups.push(quote! {
                        self.#field_ident.inline_keyboard_markup(#data)
                    });
                    schema.extend(button_schema(data, click_handler));
                }
                // User-defined types
                _ => todo!(),
            }
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
        syn::Type::Path(TypePath { path, .. }) => path.segments.last().unwrap().ident.to_string(),
        _ => panic!("Unable to get the type name for: {ty:#?}"),
    }
}
