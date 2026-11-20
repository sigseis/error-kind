// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Expr, Ident, LitStr, Result, Token, parse_quote};
use typed_ident::presets::unicode::CamelIdent;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
// Used when something is parsed like a format operation (#[error("...", ...)]).
// -----------------------------------------------------------------------------
pub struct ErrorFmt {
    pub format: LitStr,
    pub values: syn::punctuated::Punctuated<Expr, Token![,]>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl ErrorFmt {
    pub fn get_expr<'a>(
        ident: &Ident,
        attrs: impl Iterator<Item = &'a Attribute>,
        format_arg: Option<Expr>,
    ) -> Result<Expr> {
        let error_attributes = attrs
            .filter(|attr| attr.path().is_ident("error"))
            .map(|attr| (attr.span(), attr.parse_args::<Self>()))
            .collect::<Vec<_>>();
        if let Some((span, _)) = error_attributes.get(1) {
            return Err(Error::new(
                *span,
                "#[derive(ErrorKind)]: more than one #[error] attribute found on the same variant",
            ));
        }
        let mut args = match error_attributes.into_iter().next() {
            Some((_, args)) => args?,
            None => {
                // TODO: This could be cleaner - typed_ident::str::words(...);
                let display_name = CamelIdent::new_boxed(ident.to_string()).unwrap();
                let display_name = display_name
                    .words()
                    .type_erased()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .to_lowercase();
                let default_suffix = format_arg.as_ref().map(|_| ": {0}").unwrap_or_default();
                let format = format!("{display_name}{default_suffix}");
                ErrorFmt {
                    format: LitStr::new(&format, Span::call_site()),
                    values: Punctuated::new(),
                }
            }
        };

        // If there's anything that looks like a formatted string,
        // just assume that there must be an argument in the formatter.
        if let Some(format_arg) = format_arg
            && args.values.is_empty()
            && args.format.value().contains("{0}")
        {
            args.values.push(format_arg);
        }

        let format = &args.format;
        let values = &args.values;
        Ok(parse_quote!(
            write!(__ErrorCode_Formatter, #format, #values)
        ))
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl Parse for ErrorFmt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            format: input.parse()?,
            values: if input.is_empty() {
                Punctuated::new()
            } else {
                input.parse::<Token![,]>()?;
                Punctuated::parse_terminated(input)?
            },
        })
    }
}
