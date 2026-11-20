// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[cfg(feature = "alloc")]
#[path = "located.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::details::StaticString;
use crate::envelopes::ErrorEnvelope;
use crate::{Context, ErrorContext, ErrorKind};
use core::fmt::{Display, Formatter};
use core::panic::Location;

// =============================================================================
// TYPES
// =============================================================================

/// An envelope that collects error locations
///
/// # Collects
///
/// - error kind
/// - location
///
/// # When To Use
///
/// When you're in a no-alloc environment, and you want some additional debug
/// information.
///
/// # Example Display
///
/// ```text
/// static error code
///   at my-crate/src/main.rs:25:16
/// ```
#[derive(Clone, Debug)]
pub struct Located<K> {
    kind: K,
    location: Option<&'static Location<'static>>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<K: ErrorKind> Located<K> {
    #[cfg(feature = "alloc")]
    #[inline]
    pub(crate) fn into_location(self) -> Option<&'static Location<'static>> {
        self.location
    }
}

// -----------------------------------------------------------------------------
impl<K> Located<K> {
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
        alloc=[Described, Diagnostic],
        core=[],
    },
    type=Located,
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> Display for Located<K> {
    #[track_caller]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.kind, f)?;
        if f.alternate()
            && let Some(location) = self.location
        {
            write!(f, "\n  at {}", location)?;
        }
        Ok(())
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> ErrorEnvelope for Located<K> {
    type Kind = K;
    type Envelope<T: ErrorKind> = Located<T>;

    #[track_caller]
    #[inline]
    fn convert<T>(self) -> Self::Envelope<T>
    where
        T: ErrorKind + From<Self::Kind>,
    {
        Located {
            kind: From::from(self.kind),
            location: self.location,
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
        Self {
            kind,
            location: Some(Location::caller()),
        }
    }

    #[track_caller]
    #[inline]
    fn with_description<S>(kind: Self::Kind, _description: S) -> Self
    where
        S: StaticString,
    {
        Self::new(kind)
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> ErrorContext for Located<K> {
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
#[cfg(feature = "alloc")]
impl<K: ErrorKind> From<K> for Located<K> {
    #[track_caller]
    #[inline]
    fn from(orig: K) -> Self {
        Self {
            kind: orig,
            location: None,
        }
    }
}

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
impl<K: ErrorKind> From<crate::envelopes::Described<K>> for Located<K> {
    #[track_caller]
    #[inline]
    fn from(orig: crate::envelopes::Described<K>) -> Self {
        Self {
            kind: orig.error_kind(),
            location: None,
        }
    }
}

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
impl<K: ErrorKind> From<crate::envelopes::Diagnostic<K>> for Located<K> {
    #[track_caller]
    #[inline]
    fn from(orig: crate::envelopes::Diagnostic<K>) -> Self {
        Self {
            kind: orig.error_kind(),
            location: orig.into_location(),
        }
    }
}
