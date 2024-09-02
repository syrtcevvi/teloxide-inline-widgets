use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Ident, Path, Type, TypePath};

use crate::{
    attribute_parameters::{
        ButtonParameters, CalendarParameters, CheckboxListParameters, RadioListParameters,
    },
    schemes::button_schema,
};

const RADIO_LIST_TYPE: &str = "RadioList";
const CHECKBOX_LIST_TYPE: &str = "CheckboxList";
const BUTTON: &str = "Button";
const CALENDAR: &str = "Calendar";

const NOOP_DATA: &str = "noop";

/// Arguments for the top-level `#[inline_widget]` struct attribute
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(inline_widget))]
struct InlineWidgetArgs {
    /// Error type
    err_ty: Path,
    /// Bot type
    bot_ty: Path,
    /// Dialogue type
    dialogue_ty: Option<Path>,
    /// Variant for the widget state
    state: Option<Path>,
    /// Layout orientation kind
    layout_orientation: Option<Path>,
}

pub struct ComponentParameters<'a> {
    pub struct_ident: &'a Ident,
    pub field_ident: &'a Ident,
    pub field_type: &'a Type,
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

        let mut widget_container_impls = quote! {};
        let mut get_inner_widgets_impl = quote! {};
        let mut schema_impl = quote! {
            dptree::entry()
        };
        let mut markups = vec![];
        let mut sizes = vec![];

        for field in fields {
            let field_ident =
                field.ident.as_ref().expect("The user-defined widget field has to be named");
            let field_type = &field.ty;
            let field_type_name = get_type_name(&field.ty);

            let component_parameters =
                &ComponentParameters { struct_ident, field_ident, field_type };

            sizes.push(quote! {
                self.#field_ident.size()
            });

            match field_type_name.as_str() {
                RADIO_LIST_TYPE => {
                    let parameters = &match RadioListParameters::from_field(field) {
                        Ok(mut parameters) => {
                            parameters.noop_data =
                                parameters.noop_data.or(Some(NOOP_DATA.to_owned()));
                            parameters
                        }
                        Err(err) => return TokenStream::from(err.write_errors()),
                    };
                    widget_container_impl(component_parameters, &mut widget_container_impls);
                    radio_list_component_impl(
                        parameters,
                        component_parameters,
                        &mut schema_impl,
                        &mut markups,
                    );
                }
                CHECKBOX_LIST_TYPE => {
                    let parameters = &match CheckboxListParameters::from_field(field) {
                        Ok(mut parameters) => {
                            parameters.noop_data =
                                parameters.noop_data.or(Some(NOOP_DATA.to_owned()));
                            parameters
                        }
                        Err(err) => return TokenStream::from(err.write_errors()),
                    };

                    widget_container_impl(component_parameters, &mut widget_container_impls);
                    checkbox_list_component_impl(
                        parameters,
                        component_parameters,
                        &mut schema_impl,
                        &mut markups,
                    );
                }
                BUTTON => {
                    let parameters = &match ButtonParameters::from_field(field) {
                        Ok(parameters) => parameters,
                        Err(err) => return TokenStream::from(err.write_errors()),
                    };

                    button_component_impl(
                        parameters,
                        component_parameters,
                        &mut schema_impl,
                        &mut markups,
                    );
                }
                CALENDAR => {
                    let parameters = &match CalendarParameters::from_field(field) {
                        Ok(mut parameters) => {
                            parameters.prev_year = parameters.prev_year.or(Some("py".to_owned()));
                            parameters.next_year = parameters.next_year.or(Some("ny".to_owned()));
                            parameters.prev_month = parameters.prev_month.or(Some("pm".to_owned()));
                            parameters.next_month = parameters.next_month.or(Some("nm".to_owned()));
                            parameters
                        }
                        Err(err) => return TokenStream::from(err.write_errors()),
                    };
                    widget_container_impl(component_parameters, &mut widget_container_impls);
                    calendar_component_impl(
                        parameters,
                        component_parameters,
                        &mut schema_impl,
                        &mut markups,
                    );
                }
                // User-defined types
                _ => {
                    schema_impl.extend(quote! {
                        .branch(<#field_type>::schema())
                    });
                    markups.push(quote! {
                        (
                            self.#field_ident.inline_keyboard_markup(&styles),
                            self.#field_ident.size()
                        )
                    });
                }
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

        let first_markup = &markups[0];
        let inline_keyboard_markup_impl = if fields.iter().count() == 1 {
            quote! {
                #first_markup.0
            }
        } else {
            quote! {
                Layout {
                    markups: vec![#(#markups),*],
                    orientation: #layout_orientation
                }.into()
            }
        };

        let update_state_impl = if let Some(state) = state {
            quote! {
                dialogue.update(
                    #state(self)
                ).await?;

                Ok(())
            }
        } else {
            quote! {
                unimplemented!()
            }
        };

        let dialogue_ty = if let Some(dialogue_ty) = dialogue_ty {
            quote! {
                #dialogue_ty
            }
        } else {
            quote! {
                ()
            }
        };

        quote! {
            #widget_container_impls

            impl GetSize for #struct_ident {
                fn size(&self) -> Size {
                    let (rows, columns) = [#(#sizes),*].iter().fold((0, 0), |required_size, size| {
                        let Size { rows, columns } = size;
                        match #layout_orientation {
                            Horizontal => (required_size.0.max(*rows), required_size.1 + columns),
                            Vertical => (required_size.0 + rows, required_size.1.max(*columns)),
                        }
                    });
                    Size { rows, columns }
                }
            }

            impl InlineWidget for #struct_ident {
                type Err = #err_ty;
                type Bot = #bot_ty;
                type Dialogue = #dialogue_ty;

                fn schema() -> teloxide::dispatching::UpdateHandler<Self::Err> {
                    #schema_impl
                }

                fn inline_keyboard_markup(&self, styles: &WidgetStyles) -> teloxide::types::InlineKeyboardMarkup {
                    #inline_keyboard_markup_impl
                }

                async fn update_state(
                    self,
                    dialogue: &Self::Dialogue
                ) -> Result<(), Self::Err> {
                    #update_state_impl
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

pub trait GetInnerWidgets {
    fn inner_widgets(&self) -> Vec<(Vec<Ident>, Path)>;
}

fn widget_container_impl(
    ComponentParameters { struct_ident, field_ident, field_type }: &ComponentParameters,
    widget_container_impls: &mut TokenStream2,
) {
    widget_container_impls.extend(quote! {
        impl WidgetContainer<#field_type> for #struct_ident {
            fn get_widget(&mut self) -> &mut #field_type {
                &mut self.#field_ident
            }
        }
    });
}

fn radio_list_component_impl(
    RadioListParameters { prefix, noop_data }: &RadioListParameters,
    ComponentParameters { struct_ident, field_ident, field_type }: &ComponentParameters,
    schema_impl: &mut TokenStream2,
    markups: &mut Vec<TokenStream2>,
) {
    let radio_list_schema_parameters = quote! {
        RadioListSchemaParameters {
            prefix: #prefix,
            noop_data: #noop_data
        }
    };
    schema_impl.extend(quote! {
        .branch(<#field_type>::schema::<#struct_ident>(&#radio_list_schema_parameters))
    });
    markups.push(quote! {
        (
            self.#field_ident.inline_keyboard_markup(&#radio_list_schema_parameters, &styles),
            self.#field_ident.size()
        )
    });
}

fn checkbox_list_component_impl(
    CheckboxListParameters { prefix, noop_data }: &CheckboxListParameters,
    ComponentParameters { struct_ident, field_ident, field_type }: &ComponentParameters,
    schema_impl: &mut TokenStream2,
    markups: &mut Vec<TokenStream2>,
) {
    let checkbox_list_schema_parameters = quote! {
        CheckboxListSchemaParameters {
            prefix: #prefix,
            noop_data: #noop_data
        }
    };
    schema_impl.extend(quote! {
        .branch(<#field_type>::schema::<#struct_ident>(&#checkbox_list_schema_parameters))
    });
    markups.push(quote! {
        (
            self.#field_ident.inline_keyboard_markup(&#checkbox_list_schema_parameters, &styles),
            self.#field_ident.size()
        )
    });
}

fn button_component_impl(
    ButtonParameters { data, click_handler }: &ButtonParameters,
    ComponentParameters { field_ident, .. }: &ComponentParameters,
    schema_impl: &mut TokenStream2,
    markups: &mut Vec<TokenStream2>,
) {
    markups.push(quote! {
        (
            self.#field_ident.inline_keyboard_markup(#data),
            self.#field_ident.size()
        )
    });
    schema_impl.extend(button_schema(data, click_handler));
}

fn calendar_component_impl(
    CalendarParameters { prev_year, next_year, prev_month, next_month, day_click_handler }: &CalendarParameters,
    ComponentParameters { struct_ident, field_ident, field_type }: &ComponentParameters,
    schema_impl: &mut TokenStream2,
    markups: &mut Vec<TokenStream2>,
) {
    // FIXME
    let calendar_schema_parameters = quote! {
        CalendarSchemaParameters {
            previous_year_data: #prev_year,
            next_year_data: #next_year,
            previous_month_data: #prev_month,
            next_month_data: #next_month,
            noop_data: #NOOP_DATA,
            day_prefix: "d_",
            weekday_prefix: "w_"
        }
    };

    schema_impl.extend(quote! {
        .branch(<#field_type>::schema::<#struct_ident>(&#calendar_schema_parameters))
    });
    markups.push(quote! {
        (
            self.#field_ident.inline_keyboard_markup(&#calendar_schema_parameters, &styles),
            self.#field_ident.size()
        )
    });
}
