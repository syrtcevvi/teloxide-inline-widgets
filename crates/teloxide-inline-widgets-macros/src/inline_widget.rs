mod impls;

use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Path, Type, TypePath};

use crate::{
    attribute_parameters::{
        ButtonParameters, CalendarParameters, CheckboxListParameters, RadioListParameters,
    },
    constants::*,
    inline_widget::impls::*,
    schemes::CalendarSchemaTypes,
};

/// Arguments for the top-level `#[inline_widget]` struct attribute.
///
/// The `bot_ty` is the most important one and is needed for every widget.
/// For instance, `dialogue_ty` and `state` are useless for the
/// `Button` widget
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(inline_widget))]
struct InlineWidgetArgs {
    /// Bot type
    bot_ty: Path,
    /// Error type
    err_ty: Path,
    /// Dialogue type
    dialogue_ty: Option<Path>,
    /// Variant for the widget state
    state: Option<Path>,
    /// Layout orientation kind
    layout_orientation: Option<Path>,
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
                BUTTON_TYPE => {
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
                CALENDAR_TYPE => {
                    let parameters = &match CalendarParameters::from_field(field) {
                        Ok(mut parameters) => {
                            parameters.day_prefix =
                                parameters.day_prefix.or(Some(calendar::DAY_PREFIX.to_owned()));
                            parameters.weekday_prefix = parameters
                                .weekday_prefix
                                .or(Some(calendar::WEEKDAY_PREFIX.to_owned()));
                            parameters.prev_year =
                                parameters.prev_year.or(Some(calendar::PREV_YEAR.to_owned()));
                            parameters.next_year =
                                parameters.next_year.or(Some(calendar::NEXT_YEAR.to_owned()));
                            parameters.prev_month =
                                parameters.prev_month.or(Some(calendar::PREV_MONTH.to_owned()));
                            parameters.next_month =
                                parameters.next_month.or(Some(calendar::NEXT_MONTH.to_owned()));
                            parameters.noop_data =
                                parameters.noop_data.or(Some(NOOP_DATA.to_owned()));
                            parameters
                        }
                        Err(err) => return TokenStream::from(err.write_errors()),
                    };
                    widget_container_impl(component_parameters, &mut widget_container_impls);
                    calendar_component_impl(
                        parameters,
                        &CalendarSchemaTypes {
                            bot_ty: bot_ty.clone(),
                            widget_ty: struct_ident.clone(),
                            dialogue_ty: dialogue_ty.clone().expect(
                                "There must be the dialogue type for the `Calendar` widget",
                            ),
                        },
                        component_parameters,
                        &mut schema_impl,
                        &mut markups,
                    );
                }
                // User-defined types
                _ => {
                    unimplemented!()
                }
            }
        }

        let layout_orientation = layout_orientation
            .map(|lo| quote! {#lo})
            .unwrap_or(quote! {LayoutOrientation::Vertical});

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

        let update_state_impl = state
            .map(|state| quote! {dialogue.update(#state(self)).await?; Ok(())})
            .unwrap_or(quote! {unimplemented!()});
        let dialogue_ty = dialogue_ty.map(|ty| quote! {#ty}).unwrap_or(quote! {()});

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
                type Bot = #bot_ty;
                type Err = #err_ty;
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
