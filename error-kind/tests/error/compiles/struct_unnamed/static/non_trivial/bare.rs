// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::SimpleErrorKind;

// =============================================================================
// TYPES UNDER TEST
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, error_kind::Error, PartialEq)]
struct BareErrorTmpl<T>(#[error_kind] SimpleErrorKind, T);

// -----------------------------------------------------------------------------
impl<T: Default> BareErrorTmpl<T> {
    #[track_caller]
    pub fn new(error_kind: SimpleErrorKind) -> Self {
        Self(error_kind, Default::default())
    }
    #[track_caller]
    pub fn with_description(error_kind: SimpleErrorKind, _s: impl Into<String>) -> Self {
        Self(error_kind, Default::default())
    }
}

// -----------------------------------------------------------------------------
// Needed for `err!` tests to run, but you don't *have* to implement this.
// -----------------------------------------------------------------------------
impl<T: std::fmt::Debug + Default> error_kind::TrivialError for BareErrorTmpl<T> {
    #[track_caller]
    fn new(error_kind: Self::Kind) -> Self {
        Self::new(error_kind)
    }
    #[track_caller]
    fn with_description<S>(error_kind: Self::Kind, description: S) -> Self
    where
        S: error_kind::details::StaticString,
    {
        Self::with_description(error_kind, description.into_owned())
    }
}

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, error_kind::Error, PartialEq)]
struct BareErrorTmplAlt(#[error_kind] SimpleErrorKind, bool);

// -----------------------------------------------------------------------------
type BareError = BareErrorTmpl<bool>;
type BareErrorAlt = BareErrorTmplAlt;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/bare.rs");
