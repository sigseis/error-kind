// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::common::{ErrorFmt, crate_path};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Error, ItemFn, Result, ReturnType, parse_quote};

// =============================================================================
// FUNCTIONS
// =============================================================================

// -----------------------------------------------------------------------------
pub fn expand_attribute_macro(args: ErrorFmt, item: TokenStream) -> Result<TokenStream2> {
    // Parse the item as a function - if we fail then this wasn't used on a function.
    let mut item = match syn::parse::<ItemFn>(item) {
        Ok(item) => item,
        Err(_) => {
            return Err(Error::new(
                Span::call_site(),
                "#[error] attribute can only be applied to functions",
            ));
        }
    };

    // Check the return type on the function, it has to be something that implements `Context`.
    let original_type = match &item.sig.output {
        ReturnType::Type(_, ty) => ty,
        ReturnType::Default => {
            return Err(Error::new(
                Span::call_site(),
                "#[error] functions require a return type that implements `Context` type (either directly, or through `Result<_, E>` or `Option<E>`)",
            ));
        }
    };

    // Replace the body of the function with a closure that will be called,
    // followed by a call to `Context` to append the context from this location.
    let crate_path = crate_path();
    let format = &args.format;
    let original_body = &item.block;
    let values = &args.values;
    *item.block = parse_quote!({
        let body = || #original_body;
        let result: #original_type = body();
        #crate_path::ErrorContext::with_context(
            result,
            || #crate_path::__private::format!(#format, #values),
        )
    });

    Ok(quote!(#item))
}
