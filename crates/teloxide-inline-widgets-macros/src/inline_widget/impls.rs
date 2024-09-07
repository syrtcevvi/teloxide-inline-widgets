use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Type};

use crate::{
    attribute_parameters::*,
    schemes::{button_schema, calendar_schema, CalendarSchemaTypes},
};

pub struct ComponentParameters<'a> {
    /// User-defined widget identifier
    pub struct_ident: &'a Ident,
    /// Subwidget name
    pub field_ident: &'a Ident,
    /// Widget type name
    pub field_type: &'a Type,
}

pub fn widget_container_impl(
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

pub fn radio_list_component_impl(
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

pub fn checkbox_list_component_impl(
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

pub fn button_component_impl(
    parameters: &ButtonParameters,
    ComponentParameters { field_ident, .. }: &ComponentParameters,
    schema_impl: &mut TokenStream2,
    markups: &mut Vec<TokenStream2>,
) {
    let data = &parameters.data;
    markups.push(quote! {
        (
            self.#field_ident.inline_keyboard_markup(#data),
            self.#field_ident.size()
        )
    });
    schema_impl.extend(button_schema(parameters));
}

pub fn calendar_component_impl(
    parameters: &CalendarParameters,
    schema_types: &CalendarSchemaTypes,
    ComponentParameters { field_ident, .. }: &ComponentParameters,
    schema_impl: &mut TokenStream2,
    markups: &mut Vec<TokenStream2>,
) {
    let CalendarParameters {
        day_prefix,
        weekday_prefix,
        prev_year,
        next_year,
        prev_month,
        next_month,
        noop_data,
        ..
    } = &parameters;

    let calendar_schema_parameters = quote! {
        CalendarSchemaParameters {
            day_prefix: #day_prefix,
            weekday_prefix: #weekday_prefix,
            previous_year_data: #prev_year,
            next_year_data: #next_year,
            previous_month_data: #prev_month,
            next_month_data: #next_month,
            noop_data: #noop_data,
        }
    };

    markups.push(quote! {
        (
            self.#field_ident.inline_keyboard_markup(&#calendar_schema_parameters, &styles),
            self.#field_ident.size()
        )
    });
    schema_impl.extend(calendar_schema(schema_types, parameters));
}
