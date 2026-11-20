// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::{AlternateErrorDomain, AlternateErrorKind, SimpleErrorDomain, SimpleErrorKind};
use error_kind::ErrorDomain;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, ErrorDomain, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[error(domains(AlternateErrorDomain, SimpleErrorDomain))]
pub enum ComplexErrorDomain {
    Alternate(AlternateErrorKind),
    Simple(SimpleErrorKind),
}
