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
struct DiagnosticErrorTmpl<K> {
    error_kind: error_kind::envelopes::Diagnostic<K>,
}

// -----------------------------------------------------------------------------
impl<K: error_kind::ErrorKind> DiagnosticErrorTmpl<K> {
    #[track_caller]
    pub fn new(error_kind: K) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self {
            error_kind: ErrorEnvelope::new(error_kind),
        }
    }
    #[track_caller]
    pub fn with_description(error_kind: K, s: impl Into<String>) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self {
            error_kind: ErrorEnvelope::with_description(error_kind, s.into()),
        }
    }
}

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, error_kind::Error, PartialEq)]
struct DiagnosticErrorTmplAlt<K> {
    #[error_kind]
    error_kind: error_kind::envelopes::Diagnostic<K>,
}

// -----------------------------------------------------------------------------
type DiagnosticError = DiagnosticErrorTmpl<SimpleErrorKind>;
type DiagnosticErrorAlt = DiagnosticErrorTmplAlt<SimpleErrorKind>;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/diagnostic.rs");
