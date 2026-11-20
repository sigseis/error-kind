// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Context;

// =============================================================================
// TYPES
// =============================================================================

/// Propose additional error context for the error to consider collecting.
///
/// The information provided to `ErrorContext` should be thought of as debugging
/// information *only*. You should never depend on any of this information
/// logically.
///
/// Whether or not this context information is collected depends on how you
/// define your [`Error`] type.
///
/// [`Error`]: crate::Error
pub trait ErrorContext {
    /// Provide human-readable context about why an error occurred.
    ///
    /// Context is collected if the error supports it with the proper
    /// [`envelopes`]. If collected, it will be printed in stack order (first
    /// piece of context added prints first, followed by the next, etc).
    ///
    /// [`envelopes`]: crate::envelopes
    fn with_context<C, F>(self, context: F) -> Self
    where
        C: Context,
        F: FnOnce() -> C;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<T: ErrorContext> ErrorContext for Option<T> {
    #[allow(clippy::manual_map)] // Messes with #[track_caller]
    #[track_caller]
    #[inline]
    fn with_context<C, F>(self, context: F) -> Self
    where
        C: Context,
        F: FnOnce() -> C,
    {
        match self {
            None => None,
            Some(err) => Some(err.with_context(context)),
        }
    }
}

// -----------------------------------------------------------------------------
impl<T, E: ErrorContext> ErrorContext for Result<T, E> {
    #[allow(clippy::manual_map)] // Messes with #[track_caller]
    #[track_caller]
    #[inline]
    fn with_context<C, F>(self, context: F) -> Self
    where
        C: Context,
        F: FnOnce() -> C,
    {
        match self {
            Ok(val) => Ok(val),
            Err(err) => Err(err.with_context(context)),
        }
    }
}
