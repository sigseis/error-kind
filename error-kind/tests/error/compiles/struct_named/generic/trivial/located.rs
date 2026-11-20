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
struct LocatedErrorTmpl<K> {
    error_kind: error_kind::envelopes::Located<K>,
}

// -----------------------------------------------------------------------------
impl<K: error_kind::ErrorKind> LocatedErrorTmpl<K> {
    #[track_caller]
    pub fn new(error_kind: K) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self {
            error_kind: ErrorEnvelope::new(error_kind),
        }
    }
    #[track_caller]
    pub fn with_description(error_kind: K, _s: impl Into<String>) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self {
            error_kind: ErrorEnvelope::new(error_kind),
        }
    }
}

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, error_kind::Error, PartialEq)]
struct LocatedErrorTmplAlt<K> {
    #[error_kind]
    error_kind: error_kind::envelopes::Located<K>,
}

// -----------------------------------------------------------------------------
type LocatedError = LocatedErrorTmpl<SimpleErrorKind>;
type LocatedErrorAlt = LocatedErrorTmplAlt<SimpleErrorKind>;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/located.rs");
