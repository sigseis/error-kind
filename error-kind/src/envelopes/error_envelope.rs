// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::details::StaticString;
use crate::{ErrorContext, ErrorKind};
use core::fmt::{Debug, Display};

// =============================================================================
// TRAITS
// =============================================================================

/// Wraps an [`ErrorKind`] collecting optional debugging information.
///
/// For more information about this trait, see the [`envelopes`] module.
///
/// [`envelopes`]: crate::envelopes
pub trait ErrorEnvelope: ErrorContext + Debug + Display {
    /// The `ErrorKind` contained within this envelope.
    type Kind: ErrorKind;

    /// How to map one `ErrorKind` to another within this envelope.
    type Envelope<K: ErrorKind>: ErrorEnvelope;

    /// Converts the underlying error kind into a compatible error kind.
    fn convert<K>(self) -> Self::Envelope<K>
    where
        K: ErrorKind + From<Self::Kind>;

    /// The error kind held within this envelope.
    fn error_kind(&self) -> Self::Kind;

    /// Constructs a new envelope with the provided `error_kind`.
    fn new(error_kind: Self::Kind) -> Self;

    /// Constructs a new envelope with the provided `error_kind` and `description`.
    fn with_description<S>(error_kind: Self::Kind, description: S) -> Self
    where
        S: StaticString;
}
