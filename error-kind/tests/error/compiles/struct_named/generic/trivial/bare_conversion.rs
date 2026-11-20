// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::{
    AlternateErrorDomain, AlternateErrorKind, ComplexErrorDomain, SimpleErrorDomain,
    SimpleErrorKind,
};

// =============================================================================
// TYPES UNDER TEST
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, error_kind::Error, PartialEq)]
#[error(domains(AlternateErrorDomain, ComplexErrorDomain, SimpleErrorDomain))]
struct BareErrorTmpl<K: error_kind::ErrorKind> {
    #[error_kind]
    error_kind: K,
}

// -----------------------------------------------------------------------------
impl<K: error_kind::ErrorKind> BareErrorTmpl<K> {
    #[track_caller]
    pub fn new(error_kind: K) -> Self {
        Self { error_kind }
    }
    #[track_caller]
    pub fn with_description(error_kind: K, _s: impl Into<String>) -> Self {
        Self { error_kind }
    }
    pub fn error_kind(&self) -> K {
        use error_kind::envelopes::ErrorEnvelope;
        self.error_kind.error_kind()
    }
}

// -----------------------------------------------------------------------------
type Error<K> = BareErrorTmpl<K>;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/conversions.rs");
