// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Error, Result, TypePath};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TraitKind {
    CoreError,
    Display,
    Error,
    ErrorContext,
    From,
    MatchInto,
    ThisError,
    TrivialError,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl TraitKind {
    pub fn from_path(path: TypePath) -> Result<Self> {
        let path = path.to_token_stream().to_string().replace(' ', "");
        match path.as_str().trim_start_matches("::") {
            "Display" => Ok(Self::Display),
            "Error" => Ok(Self::Error),
            "ErrorContext" => Ok(Self::ErrorContext),
            "From" => Ok(Self::From),
            "MatchInto" => Ok(Self::MatchInto),
            "TrivialError" => Ok(Self::TrivialError),
            "core::convert::From" => Ok(Self::From),
            "core::error::Error" => Ok(Self::CoreError),
            "core::fmt::Display" => Ok(Self::Display),
            "error_kind::Error" => Ok(Self::ThisError),
            "error_kind::ErrorContext" => Ok(Self::ErrorContext),
            "error_kind::MatchInto" => Ok(Self::MatchInto),
            "error_kind::TrivialError" => Ok(Self::TrivialError),
            "std::convert::From" => Ok(Self::From),
            "std::error::Error" => Ok(Self::CoreError),
            "std::fmt::Display" => Ok(Self::Display),
            _ => Err(Error::new(
                path.span(),
                "#[derive(Error)]: unknown or improperly-qualified trait",
            )),
        }
    }
}
