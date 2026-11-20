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
struct DescribedErrorTmpl<T>(
    #[error_kind] error_kind::envelopes::Described<SimpleErrorKind>,
    T,
);

// -----------------------------------------------------------------------------
impl<T: Default> DescribedErrorTmpl<T> {
    #[track_caller]
    pub fn new(error_kind: SimpleErrorKind) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self(ErrorEnvelope::new(error_kind), Default::default())
    }
    #[track_caller]
    pub fn with_description(error_kind: SimpleErrorKind, s: impl Into<String>) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self(
            ErrorEnvelope::with_description(error_kind, s.into()),
            Default::default(),
        )
    }
}

// -----------------------------------------------------------------------------
// Needed for `err!` tests to run, but you don't *have* to implement this.
// -----------------------------------------------------------------------------
impl<T: std::fmt::Debug + Default> error_kind::TrivialError for DescribedErrorTmpl<T> {
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
struct DescribedErrorTmplAlt(
    #[error_kind] error_kind::envelopes::Described<SimpleErrorKind>,
    bool,
);

// -----------------------------------------------------------------------------
type DescribedError = DescribedErrorTmpl<bool>;
type DescribedErrorAlt = DescribedErrorTmplAlt;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/described.rs");
