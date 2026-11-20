// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::ErrorFmt;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Arm, DataEnum, DeriveInput, Error, Expr, Fields, Result, Variant, parse_quote};

// =============================================================================
// COMMON ITEMS
// =============================================================================

// -----------------------------------------------------------------------------
fn expand_fmt_match_arm(_item: &DeriveInput, variant: &Variant) -> Result<Arm> {
    let variant_name = &variant.ident;
    let expr = ErrorFmt::get_expr(&variant.ident, variant.attrs.iter(), None)?;
    const ERROR: &str =
        "#[derive(ErrorKind)]: currently, non-unit types are not supported for `ErrorKind`";
    match &variant.fields {
        Fields::Unit => (),
        Fields::Named(fields) => return Err(Error::new(fields.span(), ERROR)),
        Fields::Unnamed(fields) => return Err(Error::new(fields.span(), ERROR)),
    }
    Ok(parse_quote!(
        Self::#variant_name => #expr
    ))
}

// -----------------------------------------------------------------------------
fn expand_fmt_match_arms(item: &DeriveInput, data: &DataEnum) -> Result<Vec<Arm>> {
    data.variants
        .iter()
        .map(|variant| expand_fmt_match_arm(item, variant))
        .collect()
}

// -----------------------------------------------------------------------------
fn expand_fmt_impl(item: &DeriveInput, data: &DataEnum) -> Result<Expr> {
    let arms = expand_fmt_match_arms(item, data)?;
    Ok(parse_quote!(
        match self {
            #( #arms ),*
        }
    ))
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
pub fn expand_display_impl(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    let fmt_impl = expand_fmt_impl(input, data)?;
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote!(
        impl #impl_generics core::fmt::Display for #ident #type_generics #where_clause {
            fn fmt(&self, __ErrorCode_Formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #fmt_impl
            }
        }
    ))
}
