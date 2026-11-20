// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::{ErrorConfig, crate_path};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Where;
use syn::{
    DeriveInput, Error, Expr, Field, Generics, Member, Result, Type, TypeParamBound, WhereClause,
    parse_quote,
};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
pub struct ErrorImpls {
    pub error_kind_member: Member,
    pub error_kind_type: Type,
    pub from_impls: Vec<TokenStream>,
    pub generics: Generics,
    pub trivial_error_impl: Option<TokenStream>,
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
pub fn check_disallowed_fields_attributes<'a>(
    fields: impl IntoIterator<Item = &'a Field>,
) -> Result<()> {
    for field in fields.into_iter() {
        for attr in field.attrs.iter() {
            if attr.path().is_ident("error") {
                return Err(Error::new(
                    attr.span(),
                    "#[derive(Error)]: the #[error] attribute is intended for use on struct item",
                ));
            }
        }
    }
    Ok(())
}

// -----------------------------------------------------------------------------
pub fn expand_error_from_impls(
    input: &DeriveInput,
    config: &ErrorConfig,
    expr: Expr,
) -> Result<Vec<TokenStream>> {
    if config.domains.is_empty() {
        return Ok(Vec::new());
    }
    let first_span = config.domains.first().unwrap().span();

    // Check that there is exactly one type to be generic over.
    match input.generics.type_params().count() {
        0 => {
            return Err(Error::new(
                first_span,
                "#[derive(Error)]: you cannot provide a domain to be generic over if your error type is not generic itself",
            ));
        }
        1 => {}
        _ => {
            return Err(Error::new(
                first_span,
                "#[derive(Error)]: this derive macro only supports generics over a single type parameter",
            ));
        }
    }

    let error_param = &input.generics.type_params().next().unwrap().ident;
    let crate_path = crate_path();
    let ident = &input.ident;
    Ok(config
        .domains
        .iter()
        .map(|domain_type| {
            let mut generics = input.generics.clone();
            let where_clause = generics.where_clause.get_or_insert_with(|| WhereClause {
                where_token: Where::default(),
                predicates: Punctuated::new(),
            });
            where_clause
                .predicates
                .push(parse_quote!(#error_param: #crate_path::details::IsNot<#domain_type>));
            where_clause
                .predicates
                .push(parse_quote!(#domain_type: ::core::convert::From<#error_param>));
            let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
            quote!(
                impl #impl_generics From<#ident #type_generics> for #ident<#domain_type> #where_clause
                {
                    #[inline]
                    fn from(value: #ident #type_generics) -> Self {
                        #expr
                    }
                }
            )
        })
        .collect())
}

// -----------------------------------------------------------------------------
pub fn get_target_field<'a>(
    fields: impl IntoIterator<Item = &'a Field>,
) -> Result<(u32, &'a Field)> {
    let mut fields = fields
        .into_iter()
        .enumerate()
        .map(|(i, f)| (i as u32, f))
        .peekable();
    let Some(mut target_field) = fields.next() else {
        return Err(Error::new(
            Span::call_site(),
            "#[derive(Error)]: at least one field needs to be present to hold the error kind",
        ));
    };

    // If there's only one field, we may assume that the only field present is the error kind.
    if fields.peek().is_none() {
        return Ok(target_field);
    }

    // If there's multiple fields, then only one of them must be marked with #[error_kind].
    for field in fields.filter(|(_, f)| has_error_kind_attr(f)) {
        if has_error_kind_attr(target_field.1) {
            return Err(Error::new(
                field.1.span(),
                "#[derive(Error)]: only one field may have the #[error_kind] attribute set",
            ));
        }
        target_field = field;
    }
    if !has_error_kind_attr(target_field.1) {
        return Err(Error::new(
            Span::call_site(),
            "#[derive(Error)]: since there's multiple fields, one must have the #[error_kind] attribute set",
        ));
    }

    Ok(target_field)
}

// -----------------------------------------------------------------------------
pub fn get_target_generics(input: &DeriveInput, error_kind_field: &Field) -> Generics {
    let mut generics = input.generics.clone();
    if !generics.type_params().next().is_some() {
        return generics;
    }

    // If there are type parameters, we need to ensure they have the proper bounds.
    // In order to implement core `Error`, we need all types to be `Debug` at least.
    let additional_bounds = vec![TypeParamBound::Trait(parse_quote!(::core::fmt::Debug))];
    for type_param in generics.type_params_mut() {
        type_param.bounds.extend(additional_bounds.clone());
    }

    // Then, in order to implement our `Error` and `Display`, we need to ensure
    // that the error_kind_field implements the `ErrorEnvelope` trait.
    let crate_path = crate_path();
    let error_kind_type = &error_kind_field.ty;
    let where_clause = generics.where_clause.get_or_insert_with(|| WhereClause {
        where_token: Where::default(),
        predicates: Default::default(),
    });
    where_clause.predicates.push(parse_quote!(
        #error_kind_type: #crate_path::envelopes::ErrorEnvelope
    ));

    generics
}

// -----------------------------------------------------------------------------
fn has_error_kind_attr(field: &Field) -> bool {
    field
        .attrs
        .iter()
        .any(|a| a.meta.path().is_ident("error_kind"))
}
