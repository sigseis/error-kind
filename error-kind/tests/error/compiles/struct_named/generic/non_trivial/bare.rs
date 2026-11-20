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
struct BareErrorTmpl<K: error_kind::ErrorKind, T> {
    #[error_kind]
    error_kind: K,
    _other_data: T,
}

// -----------------------------------------------------------------------------
impl<K: error_kind::ErrorKind, T: Default> BareErrorTmpl<K, T> {
    #[track_caller]
    pub fn new(error_kind: K) -> Self {
        Self {
            error_kind,
            _other_data: Default::default(),
        }
    }
    #[track_caller]
    pub fn with_description(error_kind: K, _s: impl Into<String>) -> Self {
        Self {
            error_kind,
            _other_data: Default::default(),
        }
    }
}

// -----------------------------------------------------------------------------
// Needed for `err!` tests to run, but you don't *have* to implement this.
// -----------------------------------------------------------------------------
impl<K: error_kind::ErrorKind, T: std::fmt::Debug + Default> error_kind::TrivialError
    for BareErrorTmpl<K, T>
{
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
struct BareErrorTmplAlt<K: error_kind::ErrorKind> {
    #[error_kind]
    error_kind: K,
    _other_data: bool,
}

// -----------------------------------------------------------------------------
type BareError = BareErrorTmpl<SimpleErrorKind, bool>;
type BareErrorAlt = BareErrorTmplAlt<SimpleErrorKind>;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/bare.rs");
