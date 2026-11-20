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
struct DescribedErrorTmpl<K: error_kind::ErrorKind> {
    #[error_kind]
    error_kind: error_kind::envelopes::Described<K>,
    _other_data: bool,
}

// -----------------------------------------------------------------------------
impl<K: error_kind::ErrorKind> DescribedErrorTmpl<K> {
    #[track_caller]
    pub fn new(error_kind: K) -> Self {
        use ::error_kind::envelopes::ErrorEnvelope;
        Self {
            error_kind: ErrorEnvelope::new(error_kind),
            _other_data: Default::default(),
        }
    }
    #[track_caller]
    pub fn with_description(error_kind: K, s: impl Into<String>) -> Self {
        use ::error_kind::envelopes::ErrorEnvelope;
        Self {
            error_kind: ErrorEnvelope::with_description(error_kind, s.into()),
            _other_data: Default::default(),
        }
    }
    pub fn error_kind(&self) -> K {
        use error_kind::envelopes::ErrorEnvelope;
        self.error_kind.error_kind()
    }
}

// -----------------------------------------------------------------------------
type Error<K> = DescribedErrorTmpl<K>;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/conversions.rs");
