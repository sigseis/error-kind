// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::get_single_field;
use crate::common::{ErrorConfig, crate_path};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DeriveInput, Result, Variant};

// =============================================================================
// COMMON ITEMS
// =============================================================================

// -----------------------------------------------------------------------------
fn expand_from_impl(input: &DeriveInput, variant: &Variant) -> Result<Vec<TokenStream>> {
    let crate_path = crate_path();
    let field = get_single_field(variant)?;
    let field_ty = &field.ty;
    let ident = &input.ident;
    let variant_name = &variant.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(vec![
        quote!(
            impl #impl_generics ::core::convert::From<#field_ty> for #ident #ty_generics #where_clause {
                #[inline]
                fn from(value: #field_ty) -> Self {
                    Self::#variant_name(value)
                }
            }
        ),
        quote!(
            impl #impl_generics #crate_path::details::IsNot<#field_ty> for #ident #ty_generics #where_clause {}
        ),
        quote!(
            impl #impl_generics #crate_path::details::IsNot<#ident #ty_generics> for #field_ty #where_clause {}
        ),
    ])
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
pub(super) fn expand_trait_impls(
    input: &DeriveInput,
    data: &DataEnum,
    config: &ErrorConfig,
) -> Result<Vec<TokenStream>> {
    let mut from_impls: Vec<_> = data
        .variants
        .iter()
        .map(|variant| expand_from_impl(input, variant))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();

    let crate_path = crate_path();
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    for domain_ty in &config.domains {
        from_impls.push(quote!(
            impl #impl_generics ::core::convert::From<#domain_ty> for #ident #ty_generics #where_clause {
                #[inline(always)]
                fn from(value: #domain_ty) -> Self {
                    #crate_path::details::MatchInto::match_into(value)
                }
            }
        ));
        from_impls.push(quote!(
            impl #impl_generics #crate_path::details::IsNot<#domain_ty> for #ident #ty_generics #where_clause {}
        ));
        from_impls.push(quote!(
            impl #impl_generics #crate_path::details::IsNot<#ident #ty_generics> for #domain_ty #where_clause {}
        ));
    }

    Ok(from_impls)
}
