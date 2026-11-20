// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Error;
use crate::details::StaticString;

// =============================================================================
// TRAITS
// =============================================================================

/// An error that is able to be constructed trivially from an [`ErrorKind`].
///
/// This trait powers the [`err!`] macro, allowing you to use type inference to
/// construct an error based solely on a provided `ErrorKind`.
///
/// It's usually implemented for you by the `Error` derive macro. But there
/// are times when the derive macro cannot automatically implement this.
///
/// See the [`Error`] trait documentation for more information.
///
/// [`ErrorKind`]: crate::ErrorKind
/// [`err!`]: crate::err
pub trait TrivialError: Error {
    /// Construct a new error with the provided `error_kind`.
    fn new(error_kind: Self::Kind) -> Self;

    /// Construct a new error with the provided `error_kind` and `description`.
    ///
    /// The description will only be captured if your error type is configured
    /// to collect description information. See [`envelopes`] for more details.
    ///
    /// [`envelopes`]: crate::envelopes
    fn with_description<S>(error_kind: Self::Kind, description: S) -> Self
    where
        S: StaticString;
}
