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
struct DescribedError {
    error_kind: error_kind::envelopes::Described<SimpleErrorKind>,
}

// -----------------------------------------------------------------------------
impl DescribedError {
    #[track_caller]
    pub fn new(error_kind: SimpleErrorKind) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self {
            error_kind: ErrorEnvelope::new(error_kind),
        }
    }
    #[track_caller]
    pub fn with_description(error_kind: SimpleErrorKind, s: impl Into<String>) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self {
            error_kind: ErrorEnvelope::with_description(error_kind, s.into()),
        }
    }
}

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, error_kind::Error, PartialEq)]
struct DescribedErrorAlt {
    #[error_kind]
    error_kind: error_kind::envelopes::Described<SimpleErrorKind>,
}

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/described.rs");
