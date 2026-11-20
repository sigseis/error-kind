// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod error_impls;
mod named;
mod unnamed;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::{ErrorConfig, TraitKind, crate_path};
use error_impls::*;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Fields, Result};

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
pub(super) fn expand_derive_macro(input: &DeriveInput) -> Result<TokenStream> {
    let data = match &input.data {
        Data::Enum(_) => {
            return Err(Error::new(
                Span::call_site(),
                "#[derive(Error)]: enums are not allowed (only structs are allowed for `Error`)",
            ));
        }
        Data::Struct(data) => data,
        Data::Union(_) => {
            return Err(Error::new(
                Span::call_site(),
                "#[derive(Error)]: unions are not allowed (only structs are allowed for `Error`)",
            ));
        }
    };

    // Check that the user didn't accidentally use #[error_kind] at item-level.
    for attr in input.attrs.iter() {
        if attr.path().is_ident("error_kind") {
            return Err(Error::new(
                attr.span(),
                "#[derive(Error)]: the #[error_kind] attribute is intended for use on struct fields",
            ));
        }
    }

    // Get the error configuration for the derive macro.
    let config = ErrorConfig::parse(input)?;

    // Produce the `Error` implementation for the type that the trait is attached to.
    let error_impl = match &data.fields {
        Fields::Named(fields) => named::expand_derive_macro(input, data, &config, fields)?,
        Fields::Unit => {
            return Err(Error::new(
                Span::call_site(),
                "#[derive(Error)]: structs must have at least one field marked with the #[error_kind] attribute",
            ));
        }
        Fields::Unnamed(fields) => unnamed::expand_derive_macro(input, data, &config, fields)?,
    };

    // Conditionally configure the final set of expanded traits.
    let crate_path = crate_path();
    let ident = &input.ident;
    let error_kind_member = &error_impl.error_kind_member;
    let error_kind_type = &error_impl.error_kind_type;
    let (impl_generics, type_generics, where_clause) = error_impl.generics.split_for_impl();
    let display_impl = config.prepare_item(
        &[TraitKind::Display],
        quote! {
            impl #impl_generics ::core::fmt::Display for #ident #type_generics #where_clause {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Display::fmt(&self.#error_kind_member, f)
                }
            }
        },
    );
    let from_impls = config.prepare_items(&[TraitKind::From], error_impl.from_impls);
    let core_error_impl = config.prepare_item(
        &[TraitKind::CoreError, TraitKind::Error],
        quote! {
            impl #impl_generics ::core::error::Error for #ident #type_generics #where_clause {}
        },
    );
    let this_error_impl = config.prepare_item(
        &[TraitKind::ThisError, TraitKind::Error],
        quote! {
            impl #impl_generics #crate_path::Error for #ident #type_generics #where_clause {
                type Kind = <#error_kind_type as #crate_path::envelopes::ErrorEnvelope>::Kind;

                #[inline]
                fn error_kind(&self) -> Self::Kind {
                    #crate_path::envelopes::ErrorEnvelope::error_kind(&self.#error_kind_member)
                }
            }
        },
    );
    let trivial_error_impl =
        config.prepare_item_opt(&[TraitKind::TrivialError], error_impl.trivial_error_impl);
    let error_context_impl = config.prepare_item(
        &[TraitKind::ErrorContext],
        quote! {
            impl #impl_generics #crate_path::ErrorContext for #ident #type_generics #where_clause {
                #[track_caller]
                #[inline]
                fn with_context<C, F>(mut self, context: F) -> Self
                where
                    C: #crate_path::Context,
                    F: FnOnce() -> C,
                {
                    self.#error_kind_member = #crate_path::ErrorContext::with_context(
                        self.#error_kind_member,
                        context,
                    );
                    self
                }
            }
        },
    );

    Ok(quote! {
        #display_impl

        #(#from_impls)*

        #core_error_impl

        #this_error_impl

        #trivial_error_impl

        #error_context_impl
    })
}
