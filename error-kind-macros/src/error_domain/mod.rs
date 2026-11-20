// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod display;
mod from;
mod match_into;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::{ErrorConfig, TraitKind, crate_path};
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Field, Fields, Result, Variant};

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
fn get_single_field(variant: &Variant) -> Result<&Field> {
    const ERROR: &str = "#[derive(ErrorDomain)]: error domains must be enums consisting solely of variants with a single unnamed `ErrorKind` field";
    let fields = match &variant.fields {
        Fields::Unnamed(fields) => fields,
        Fields::Named(fields) => {
            return Err(Error::new(fields.span(), ERROR));
        }
        Fields::Unit => {
            return Err(Error::new(variant.span(), ERROR));
        }
    };
    let mut iter = fields.unnamed.iter();
    let Some(first) = iter.next() else {
        return Err(Error::new(
            fields.span(),
            "#[derive(ErrorDomain)]: unnamed variants must have exactly 1 field (the wrapped error kind)",
        ));
    };
    if let Some(next) = iter.next() {
        return Err(Error::new(
            next.span(),
            "#[derive(ErrorDomain)]: unnamed variants must have exactly 1 field (the wrapped error kind)",
        ));
    }
    Ok(first)
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
pub(super) fn expand_derive_macro(input: &DeriveInput) -> Result<TokenStream> {
    let data = match &input.data {
        Data::Enum(data) => data,
        Data::Struct(_) => {
            return Err(Error::new(
                Span::call_site(),
                "#[derive(ErrorDomain)]: structs are not allowed\nonly enum types are allowed for `ErrorDomain`",
            ));
        }
        Data::Union(_) => {
            return Err(Error::new(
                Span::call_site(),
                "#[derive(ErrorDomain)]: unions are not allowed\nonly enum types are allowed for `ErrorDomain`",
            ));
        }
    };
    if data.variants.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[derive(ErrorDomain)]: cannot derive on an empty enum",
        ));
    }

    let config = ErrorConfig::parse(input)?;
    let display_impl = config.prepare_item(
        &[TraitKind::Display],
        display::expand_trait_impl(input, data)?,
    );
    let match_into_impl = config.prepare_item(
        &[TraitKind::MatchInto],
        match_into::expand_trait_impl(input, data)?,
    );
    let from_impls = from::expand_trait_impls(input, data, &config)?;
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let crate_path = crate_path();
    Ok(quote!(
        impl #impl_generics #crate_path::ErrorDomain for #ident #type_generics #where_clause {}

        #display_impl

        #match_into_impl

        #(#from_impls)*
    ))
}
