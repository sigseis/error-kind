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
struct LocatedErrorTmpl<T> {
    #[error_kind]
    error_kind: error_kind::envelopes::Located<SimpleErrorKind>,
    _other_data: T,
}

// -----------------------------------------------------------------------------
impl<T: Default> LocatedErrorTmpl<T> {
    #[track_caller]
    pub fn new(error_kind: SimpleErrorKind) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self {
            error_kind: ErrorEnvelope::new(error_kind),
            _other_data: Default::default(),
        }
    }
    #[track_caller]
    pub fn with_description(error_kind: SimpleErrorKind, s: impl Into<String>) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self {
            error_kind: ErrorEnvelope::with_description(error_kind, s.into()),
            _other_data: Default::default(),
        }
    }
}

// -----------------------------------------------------------------------------
// Needed for `err!` tests to run, but you don't *have* to implement this.
// -----------------------------------------------------------------------------
impl<T: std::fmt::Debug + Default> error_kind::TrivialError for LocatedErrorTmpl<T> {
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
struct LocatedErrorTmplAlt {
    #[error_kind]
    error_kind: error_kind::envelopes::Located<SimpleErrorKind>,
    _other_data: bool,
}

// -----------------------------------------------------------------------------
type LocatedError = LocatedErrorTmpl<bool>;
type LocatedErrorAlt = LocatedErrorTmplAlt;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/located.rs");
