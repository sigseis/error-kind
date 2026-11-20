// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::TraitKind;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::{Attribute, DeriveInput, Result, Token, TypePath, parenthesized};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
// Used when parsing error attributes on items (#[error(config(...), ...)]).
// -----------------------------------------------------------------------------
#[derive(Default)]
pub struct ErrorConfig {
    pub disable: Vec<TraitKind>,
    pub domains: Vec<TypePath>,
    pub hidden: Vec<TraitKind>,
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
// Private Helpers:
// -----------------------------------------------------------------------------
impl ErrorConfig {
    fn parse_attr(attr: &Attribute) -> Result<Self> {
        fn adapt(iter: impl Iterator<Item = TypePath>) -> Result<Vec<TraitKind>> {
            iter.map(TraitKind::from_path).collect()
        }
        let mut config = Self::default();
        attr.parse_nested_meta(|meta| {
            let content;
            parenthesized!(content in meta.input);
            let traits = content
                .parse_terminated(TypePath::parse, Token![,])?
                .into_iter();
            match &meta.path {
                p if p.is_ident("disable") => config.disable.extend(adapt(traits)?),
                p if p.is_ident("domains") => config.domains.extend(traits),
                p if p.is_ident("hidden") => config.hidden.extend(adapt(traits)?),
                _ => {
                    return Err(meta.error(
                        "unsupported meta argument, expected one of: disable, domains, hidden",
                    ));
                }
            };
            Ok(())
        })?;
        Ok(config)
    }
}

// -----------------------------------------------------------------------------
// Public API:
// -----------------------------------------------------------------------------
impl ErrorConfig {
    pub fn parse(input: &DeriveInput) -> Result<Self> {
        let mut config = Self::default();
        for attr in input.attrs.iter() {
            if !attr.path().is_ident("error") {
                continue;
            }
            let next = Self::parse_attr(attr)?;
            config.disable.extend(next.disable);
            config.hidden.extend(next.hidden);
            config.domains.extend(next.domains);
        }
        Ok(config)
    }

    pub fn prepare_item(
        &self,
        item_kinds: &[TraitKind],
        item_impl: TokenStream,
    ) -> Option<TokenStream> {
        self.prepare_item_opt(item_kinds, Some(item_impl))
    }

    pub fn prepare_item_opt(
        &self,
        item_kinds: &[TraitKind],
        item_impl: Option<TokenStream>,
    ) -> Option<TokenStream> {
        let item_impl = item_impl?;
        if item_kinds.iter().any(|t| self.disable.contains(t)) {
            return None;
        }
        if item_kinds.iter().any(|t| self.hidden.contains(t)) {
            return Some(quote! {
                #[doc(hidden)]
                #item_impl
            });
        }
        Some(item_impl)
    }

    pub fn prepare_items(
        &self,
        item_kinds: &[TraitKind],
        item_impls: Vec<TokenStream>,
    ) -> Vec<TokenStream> {
        if item_impls.is_empty() {
            return item_impls;
        }
        if item_kinds.iter().any(|t| self.disable.contains(t)) {
            return Vec::new();
        }
        if item_kinds.iter().any(|t| self.hidden.contains(t)) {
            return item_impls
                .into_iter()
                .map(|item_impl| {
                    quote! {
                        #[doc(hidden)]
                        #item_impl
                    }
                })
                .collect();
        }
        item_impls
    }
}
