// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "described.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::details::StaticString;
use crate::envelopes::{Diagnostic, ErrorEnvelope, Located};
use crate::{Context, ErrorContext, ErrorKind};
use core::fmt::{Display, Formatter};
use std_alloc::string::String;

// =============================================================================
// TYPES
// =============================================================================

/// An envelope that collects error descriptions
///
/// # Collects
///
/// - description
/// - error kind
///
/// # When To Use
///
/// When you primarily want to return error codes, but you also think some
/// detailed error string is okay to pass back to the client.
///
/// # Example Display
///
/// ```text
/// static error code: detailed dynamic error message
/// ```
#[derive(Clone, Debug)]
pub struct Described<K> {
    description: String,
    kind: K,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<K: ErrorKind> Described<K> {
    #[inline]
    pub(crate) fn into_description(self) -> String {
        self.description
    }
}

// -----------------------------------------------------------------------------
impl<K> Described<K> {
    #[inline]
    pub(crate) fn error_kind_ref(&self) -> &K {
        &self.kind
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
crate::envelopes::impl_error_adapter_traits! {
    others={
        alloc=[Diagnostic],
        core=[Located],
    },
    type=Described,
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> Display for Described<K> {
    #[track_caller]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.kind, f)?;
        if !self.description.is_empty() {
            write!(f, ": {}", self.description)?;
        }
        Ok(())
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> ErrorEnvelope for Described<K> {
    type Kind = K;
    type Envelope<T: ErrorKind> = Described<T>;

    #[track_caller]
    #[inline]
    fn convert<T>(self) -> Self::Envelope<T>
    where
        T: ErrorKind + From<Self::Kind>,
    {
        Described {
            description: self.description,
            kind: From::from(self.kind),
        }
    }

    #[track_caller]
    #[inline]
    fn error_kind(&self) -> Self::Kind {
        self.kind
    }

    #[track_caller]
    #[inline]
    fn new(kind: Self::Kind) -> Self {
        Self::with_description(kind, String::new())
    }

    #[track_caller]
    #[inline]
    fn with_description<S>(kind: Self::Kind, description: S) -> Self
    where
        S: StaticString,
    {
        Self {
            description: description.into_owned(),
            kind,
        }
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> ErrorContext for Described<K> {
    #[track_caller]
    #[inline(always)]
    fn with_context<C, F>(self, _context: F) -> Self
    where
        C: Context,
        F: FnOnce() -> C,
    {
        self
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> From<K> for Described<K> {
    #[inline]
    fn from(orig: K) -> Self {
        Self {
            kind: orig,
            description: String::new(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> From<Diagnostic<K>> for Described<K> {
    #[track_caller]
    #[inline]
    fn from(orig: Diagnostic<K>) -> Self {
        Self {
            kind: orig.error_kind(),
            description: orig.into_description(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> From<Located<K>> for Described<K> {
    #[track_caller]
    #[inline]
    fn from(orig: Located<K>) -> Self {
        Self {
            kind: orig.error_kind(),
            description: String::new(),
        }
    }
}
