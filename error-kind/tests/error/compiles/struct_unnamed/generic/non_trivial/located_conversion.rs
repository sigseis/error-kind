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
struct BareErrorTmpl<K: error_kind::ErrorKind>(
    #[error_kind] error_kind::envelopes::Located<K>,
    bool,
);

// -----------------------------------------------------------------------------
impl<K: error_kind::ErrorKind> BareErrorTmpl<K> {
    #[track_caller]
    pub fn new(error_kind: K) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self(ErrorEnvelope::new(error_kind), Default::default())
    }
    #[track_caller]
    pub fn with_description(error_kind: K, s: impl Into<String>) -> Self {
        use error_kind::envelopes::ErrorEnvelope;
        Self(
            ErrorEnvelope::with_description(error_kind, s.into()),
            Default::default(),
        )
    }
    pub fn error_kind(&self) -> K {
        use error_kind::envelopes::ErrorEnvelope;
        self.0.error_kind()
    }
}

// -----------------------------------------------------------------------------
type Error<K> = BareErrorTmpl<K>;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
include!("../../../templates/conversions.rs");
