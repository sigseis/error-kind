// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::details::StaticString;
use crate::envelopes::ErrorEnvelope;
use crate::{Context, ErrorContext};
use core::fmt::{Debug, Display};

// =============================================================================
// TRAITS
// =============================================================================

/// A marker trait for a type that denotes a kind of error that took place.
///
/// These types are expected to be very simple enums, which contain only unit
/// variants denoting the kind of error that occurred.
///
/// If you want to compose an `ErrorKind` which is a combination of multiple
/// different `ErrorKind`, see the [`ErrorDomain`] trait.
///
/// [`ErrorDomain`]: crate::ErrorDomain
///
/// # Display Formatting
///
/// The derive macro for this trait will automatically implement `Display` for
/// you. The default process is to break the variant name into words, lowercase
/// it, and then separate the words by spaces.
///
/// ```rust
/// # use error_kind::ErrorKind;
/// #[derive(Clone, Copy, Debug, ErrorKind)]
/// enum IoErrorKind {
///     BadFileDescriptor, // Prints "bad file descriptor"
///     FileNotFound,      // Prints "file not found"
///     // ...
/// }
/// ```
///
/// If you don't like a particular display format, you can change it with the
/// `#[error(...)]` attribute on a variant.
///
/// ```rust
/// # use error_kind::ErrorKind;
/// #[derive(Clone, Copy, Debug, ErrorKind)]
/// enum IoErrorKind {
///     BadFileDescriptor, // Prints "bad file descriptor"
///     #[error("custom display")]
///     FileNotFound,      // Prints "custom display"
///     // ...
/// }
/// ```
///
/// # Examples
///
/// An error kind denotes a set of semantically related errors.
///
/// ```rust
/// # use error_kind::ErrorKind;
/// #[derive(Clone, Copy, Debug, ErrorKind)]
/// enum IoErrorKind {
///     BadFileDescriptor,
///     FileNotFound,
///     // ...
/// }
/// ```
///
/// You can define multiple kinds of errors by creating separate enums.
///
/// ```rust
/// # use error_kind::ErrorKind;
/// #[derive(Clone, Copy, Debug, ErrorKind)]
/// enum IoErrorKind {
///     BadFileDescriptor,
///     FileNotFound,
///     // ...
/// }
///
/// #[derive(Clone, Copy, Debug, ErrorKind)]
/// enum ParseErrorKind {
///     InvalidFormat,
///     UnexpectedEndOfData,
///     // ...
/// }
/// ```
pub trait ErrorKind: Clone + Copy + Debug + Display {}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<K: ErrorKind> ErrorEnvelope for K {
    type Kind = Self;
    type Envelope<T: ErrorKind> = T;

    #[track_caller]
    #[inline(always)]
    fn convert<T>(self) -> Self::Envelope<T>
    where
        T: ErrorKind + From<Self::Kind>,
    {
        From::from(self)
    }

    #[inline(always)]
    fn error_kind(&self) -> Self::Kind {
        *self
    }

    #[inline(always)]
    fn new(kind: Self::Kind) -> Self {
        kind
    }

    #[inline(always)]
    fn with_description<S>(kind: Self::Kind, _description: S) -> Self
    where
        S: StaticString,
    {
        kind
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> ErrorContext for K {
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
