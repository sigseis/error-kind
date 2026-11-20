#![doc = include_str!("README.md")]

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod attribute;
mod common;
mod error;
mod error_domain;
mod error_kind;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::ErrorFmt;
use proc_macro::TokenStream;
use syn::{DeriveInput, Error, parse_macro_input};

// =============================================================================
// PROC MACRO ENTRY FUNCTIONS
// =============================================================================

/// Derive macro available if error-kind is built with `features = ["macros"]`
#[proc_macro_derive(ErrorDomain, attributes(error))]
pub fn derive_error_domain(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    error_domain::expand_derive_macro(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive macro available if error-kind is built with `features = ["macros"]`
#[proc_macro_derive(ErrorKind, attributes(error))]
pub fn derive_error_kind(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    error_kind::expand_derive_macro(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive macro available if error-kind is built with `features = ["macros"]`
#[proc_macro_derive(Error, attributes(error, error_kind))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    error::expand_derive_macro(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Attribute macro available if error-kind is built with `features = ["macros"]`
#[proc_macro_attribute]
pub fn error(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ErrorFmt);
    attribute::expand_attribute_macro(args, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
