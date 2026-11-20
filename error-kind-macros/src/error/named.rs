// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::{ErrorConfig, crate_path};
use crate::error::{
    ErrorImpls, check_disallowed_fields_attributes, expand_error_from_impls, get_target_field,
    get_target_generics,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Field, FieldsNamed, Generics, Member, Result, parse_quote};

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
fn expand_from_impls(
    input: &DeriveInput,
    _data: &DataStruct,
    config: &ErrorConfig,
    fields: &FieldsNamed,
    error_field: &Field,
) -> Result<Vec<TokenStream>> {
    let crate_path = crate_path();
    let field_assignments = fields.named.iter().map(|field| {
        let field_name = &field.ident;
        if field.ident == error_field.ident {
            quote! {
                #field_name: #crate_path::envelopes::ErrorEnvelope::convert(value.#field_name)
            }
        } else {
            quote! {
                #field_name: value.#field_name
            }
        }
    });
    expand_error_from_impls(
        input,
        config,
        parse_quote!(Self { #(#field_assignments,)* }),
    )
}

// -----------------------------------------------------------------------------
fn expand_trivial_error_impl(
    input: &DeriveInput,
    _data: &DataStruct,
    fields: &FieldsNamed,
    field: &Field,
    generics: &Generics,
) -> Option<TokenStream> {
    if fields.named.len() != 1 {
        return None;
    }
    let crate_path = crate_path();
    let field_name = &field.ident;
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    Some(quote!(
        impl #impl_generics #crate_path::TrivialError for #ident #type_generics #where_clause {
            #[track_caller]
            #[inline]
            fn new(error_kind: Self::Kind) -> Self {
                Self {
                    #field_name: #crate_path::envelopes::ErrorEnvelope::new(
                        error_kind,
                    ),
                }
            }

            #[track_caller]
            #[inline]
            fn with_description<S>(error_kind: Self::Kind, description: S) -> Self
            where
                S: #crate_path::details::StaticString,
            {
                Self {
                    #field_name: #crate_path::envelopes::ErrorEnvelope::with_description(
                        error_kind,
                        description,
                    ),
                }
            }
        }
    ))
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
pub(super) fn expand_derive_macro(
    input: &DeriveInput,
    data: &DataStruct,
    config: &ErrorConfig,
    fields: &FieldsNamed,
) -> Result<ErrorImpls> {
    check_disallowed_fields_attributes(fields.named.iter())?;
    let (_, field) = get_target_field(&fields.named)?;
    let generics = get_target_generics(input, field);
    let error_kind_member = Member::Named(field.ident.clone().unwrap());
    Ok(ErrorImpls {
        error_kind_member,
        error_kind_type: field.ty.clone(),
        from_impls: expand_from_impls(input, data, config, fields, field)?,
        trivial_error_impl: expand_trivial_error_impl(input, data, fields, field, &generics),
        generics,
    })
}
