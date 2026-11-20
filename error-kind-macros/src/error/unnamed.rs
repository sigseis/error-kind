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
use syn::spanned::Spanned;
use syn::{DataStruct, DeriveInput, FieldsUnnamed, Generics, Index, Member, Result, parse_quote};

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
fn expand_from_impls(
    input: &DeriveInput,
    _data: &DataStruct,
    config: &ErrorConfig,
    fields: &FieldsUnnamed,
    error_field: u32,
) -> Result<Vec<TokenStream>> {
    let crate_path = crate_path();
    let field_assignments = fields.unnamed.iter().enumerate().map(|(idx, field)| {
        let accessor = Member::Unnamed(Index {
            index: idx.try_into().unwrap(),
            span: field.span(),
        });
        if idx == error_field.try_into().unwrap() {
            quote! {
                #crate_path::envelopes::ErrorEnvelope::convert(value.#accessor)
            }
        } else {
            quote! {
                value.#accessor
            }
        }
    });
    expand_error_from_impls(input, config, parse_quote!(Self(#(#field_assignments,)*)))
}

// -----------------------------------------------------------------------------
fn expand_trivial_error_impl(
    input: &DeriveInput,
    _data: &DataStruct,
    fields: &FieldsUnnamed,
    generics: &Generics,
) -> Option<TokenStream> {
    if fields.unnamed.len() != 1 {
        return None;
    }
    let crate_path = crate_path();
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    Some(quote!(
        impl #impl_generics #crate_path::TrivialError for #ident #type_generics #where_clause {
            #[track_caller]
            #[inline]
            fn new(error_kind: Self::Kind) -> Self {
                Self(#crate_path::envelopes::ErrorEnvelope::new(error_kind))
            }

            #[track_caller]
            #[inline]
            fn with_description<S>(error_kind: Self::Kind, description: S) -> Self
            where
                S: #crate_path::details::StaticString,
            {
                Self(#crate_path::envelopes::ErrorEnvelope::with_description(error_kind, description))
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
    fields: &FieldsUnnamed,
) -> Result<ErrorImpls> {
    check_disallowed_fields_attributes(fields.unnamed.iter())?;
    let (index, field) = get_target_field(&fields.unnamed)?;
    let generics = get_target_generics(input, field);
    let error_kind_member = Member::Unnamed(Index {
        index,
        span: field.span(),
    });
    Ok(ErrorImpls {
        error_kind_member,
        error_kind_type: field.ty.clone(),
        from_impls: expand_from_impls(input, data, config, fields, index)?,
        trivial_error_impl: expand_trivial_error_impl(input, data, fields, &generics),
        generics,
    })
}
