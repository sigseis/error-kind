// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "static_string.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
use std_alloc::{borrow::Cow, string::String};

// =============================================================================
// SEALED
// =============================================================================

// -----------------------------------------------------------------------------
mod sealed {
    pub trait Sealed {}
    impl Sealed for &'static str {}
    #[cfg(feature = "alloc")]
    impl Sealed for super::Cow<'static, str> {}
    #[cfg(feature = "alloc")]
    impl Sealed for super::String {}
}

// =============================================================================
// TYPES
// =============================================================================

/// Allow for access and conversion from a static string asset.
///
/// This is useful for allowing a relaxed reliance on the `alloc` crate.
/// If the `alloc` feature is not enabled, you cannot use types that would
/// require taking ownership of an allocated string.
///
/// This is to explicitly avoid naming `String` in places.
pub trait StaticString: sealed::Sealed + Sized + Send + Sync + 'static {
    /// Take CoW ownership of the provided static string.
    #[cfg(feature = "alloc")]
    fn into_cow(self) -> Cow<'static, str>;

    /// Take ownership of the provided static string.
    #[cfg(feature = "alloc")]
    fn into_owned(self) -> String;

    /// Attempt to borrow the string as a static string reference.
    ///
    /// This can fail if the type this is implemented over does not contain
    /// a static reference to a string (e.g. it's an allocated `String`).
    fn try_borrow(&self) -> Option<&'static str>;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl StaticString for &'static str {
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn into_cow(self) -> Cow<'static, str> {
        Cow::Borrowed(self)
    }

    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn into_owned(self) -> String {
        String::from(self)
    }

    #[inline(always)]
    fn try_borrow(&self) -> Option<&'static str> {
        Some(self)
    }
}

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
impl StaticString for Cow<'static, str> {
    #[inline(always)]
    fn into_cow(self) -> Cow<'static, str> {
        self
    }

    #[inline]
    fn into_owned(self) -> String {
        self.into_owned()
    }

    #[inline]
    fn try_borrow(&self) -> Option<&'static str> {
        match self {
            Cow::Borrowed(value) => Some(value),
            Cow::Owned(_) => None,
        }
    }
}

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
impl StaticString for String {
    #[inline(always)]
    fn into_cow(self) -> Cow<'static, str> {
        Cow::Owned(self)
    }

    #[inline(always)]
    fn into_owned(self) -> String {
        self
    }

    #[inline(always)]
    fn try_borrow(&self) -> Option<&'static str> {
        None
    }
}
