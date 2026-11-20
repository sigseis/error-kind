// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod display;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::{ErrorConfig, TraitKind, crate_path};
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Result};

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
                "#[derive(ErrorKind)]: structs are not allowed\nonly enum types are allowed for `ErrorKind`",
            ));
        }
        Data::Union(_) => {
            return Err(Error::new(
                Span::call_site(),
                "#[derive(ErrorKind)]: unions are not allowed\nonly enum types are allowed for `ErrorKind`",
            ));
        }
    };

    if data.variants.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[derive(ErrorKind)]: cannot derive on an empty enum",
        ));
    }

    let config = ErrorConfig::parse(input)?;
    let display_impl = config.prepare_item(
        &[TraitKind::Display],
        display::expand_display_impl(input, data)?,
    );
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let crate_path = crate_path();
    Ok(quote!(
        impl #impl_generics #crate_path::ErrorKind for #ident #type_generics #where_clause {}

        #display_impl
    ))
}
