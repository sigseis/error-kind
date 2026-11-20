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
struct DiagnosticError(error_kind::envelopes::Diagnostic<SimpleErrorKind>);

// -----------------------------------------------------------------------------
impl DiagnosticError {
    #[track_caller]
    pub fn new(error_kind: SimpleErrorKind) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self(ErrorEnvelope::new(error_kind))
    }
    #[track_caller]
    pub fn with_description(error_kind: SimpleErrorKind, s: impl Into<String>) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self(ErrorEnvelope::with_description(error_kind, s.into()))
    }
}

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, error_kind::Error, PartialEq)]
struct DiagnosticErrorAlt(#[error_kind] error_kind::envelopes::Diagnostic<SimpleErrorKind>);

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/diagnostic.rs");
