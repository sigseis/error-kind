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
struct DescribedErrorTmpl<K>(error_kind::envelopes::Described<K>);

// -----------------------------------------------------------------------------
impl<K: error_kind::ErrorKind> DescribedErrorTmpl<K> {
    #[track_caller]
    pub fn new(error_kind: K) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self(ErrorEnvelope::new(error_kind))
    }
    #[track_caller]
    pub fn with_description(error_kind: K, s: impl Into<String>) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self(ErrorEnvelope::with_description(error_kind, s.into()))
    }
}

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, error_kind::Error, PartialEq)]
struct DescribedErrorTmplAlt<K>(#[error_kind] error_kind::envelopes::Described<K>);

// -----------------------------------------------------------------------------
type DescribedError = DescribedErrorTmpl<SimpleErrorKind>;
type DescribedErrorAlt = DescribedErrorTmplAlt<SimpleErrorKind>;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/described.rs");
