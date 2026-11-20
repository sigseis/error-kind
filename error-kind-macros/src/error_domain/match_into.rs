// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::get_single_field;
use crate::common::crate_path;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Arm, DataEnum, DeriveInput, Expr, Result, Type, Variant, parse_quote};

// =============================================================================
// COMMON ITEMS
// =============================================================================

// -----------------------------------------------------------------------------
fn expand_match_into_impl_arm(_item: &DeriveInput, variant: &Variant) -> Result<Arm> {
    let variant_name = &variant.ident;
    Ok(parse_quote!(
        Self::#variant_name(kind) => T::from(kind)
    ))
}

// -----------------------------------------------------------------------------
fn expand_match_into_impl_arms(item: &DeriveInput, data: &DataEnum) -> Result<Vec<Arm>> {
    data.variants
        .iter()
        .map(|variant| expand_match_into_impl_arm(item, variant))
        .collect()
}

// -----------------------------------------------------------------------------
fn expand_match_into_impl(item: &DeriveInput, data: &DataEnum) -> Result<Expr> {
    let arms = expand_match_into_impl_arms(item, data)?;
    Ok(parse_quote!(
        match self {
            #( #arms ),*
        }
    ))
}

// -----------------------------------------------------------------------------
fn expand_match_into_from_type(_item: &DeriveInput, variant: &Variant) -> Result<Type> {
    let field = get_single_field(variant)?;
    Ok(field.ty.clone())
}

// -----------------------------------------------------------------------------
fn expand_match_into_from_types(item: &DeriveInput, data: &DataEnum) -> Result<Vec<Type>> {
    data.variants
        .iter()
        .map(|variant| expand_match_into_from_type(item, variant))
        .collect()
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
pub(super) fn expand_trait_impl(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    let crate_path = crate_path();
    let from_types = expand_match_into_from_types(input, data)?;
    let match_into_impl = expand_match_into_impl(input, data)?;
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote!(
        impl<T: #(From<#from_types>)+*> #impl_generics #crate_path::details::MatchInto<T> for #ident #type_generics #where_clause {
            #[inline]
            fn match_into(self) -> T {
                #match_into_impl
            }
        }
    ))
}
