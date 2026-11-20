// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::{Context, Error, ErrorKind, TrivialError};

// =============================================================================
// SEALED
// =============================================================================

// -----------------------------------------------------------------------------
mod sealed {
    pub trait Sealed {}
    impl<E: super::Error> Sealed for Option<E> {}
}

// =============================================================================
// TRAITS
// =============================================================================

/// Extension functions for `Option<T>` when `T` is an [`Error`].
pub trait OptionExt: sealed::Sealed {
    /// The error type associated with this option (`Some(_)`).
    type Error: Error;

    /// Convert an option from some foreign error, capturing the error context.
    ///
    /// This is equivalent to the following code:
    ///
    /// ```
    /// # use error_kind::{Error, ErrorContext, ErrorKind, ResultExt, TrivialError};
    /// # #[derive(Clone, Copy, Debug, ErrorKind)]
    /// # enum MyErrorKind { Variant }
    /// # #[derive(Clone, Copy, Debug, Error)]
    /// # struct TargetError(MyErrorKind);
    /// # let option = Option::<std::io::Error>::None;
    /// # let error_kind = MyErrorKind::Variant;
    /// # let end_option: Option<TargetError> =
    /// match option {
    ///     Some(error) => Some(
    ///         TargetError::new(error_kind.into()).with_context(|| error)
    ///     ),
    ///     None => None,
    /// }
    /// # ;
    /// ```
    fn capture<E, K>(error_kind: K, option: Option<E>) -> Self
    where
        E: Context,
        K: ErrorKind + Into<<Self::Error as Error>::Kind>,
        Self::Error: TrivialError;

    /// Convert an option from some foreign error.
    ///
    /// This is equivalent to the following code:
    ///
    /// ```
    /// # use error_kind::{Error, ErrorContext, ErrorKind, ResultExt, TrivialError};
    /// # #[derive(Clone, Copy, Debug, ErrorKind)]
    /// # enum MyErrorKind { Variant }
    /// # #[derive(Clone, Copy, Debug, Error)]
    /// # struct TargetError(MyErrorKind);
    /// # let option = Option::<std::io::Error>::None;
    /// # let error_kind = MyErrorKind::Variant;
    /// # let end_option: Option<TargetError> =
    /// match option {
    ///     Some(error) => Some(TargetError::new(error_kind.into())),
    ///     None => None,
    /// }
    /// # ;
    /// ```
    fn convert<E, K>(error_kind: K, option: Option<E>) -> Self
    where
        E: Context,
        K: ErrorKind + Into<<Self::Error as Error>::Kind>,
        Self::Error: TrivialError;

    /// Provide human-readable information about why an error occurred.
    ///
    /// This is a wrapper which redirects to [`ErrorContext::with_context`] for
    /// the wrapped `Some(value)`. If the `Option` is `None`, this information
    /// is discarded.
    ///
    /// [`ErrorContext::with_context`]: crate::ErrorContext::with_context
    fn with_context<C, F>(self, context: F) -> Self
    where
        C: Context,
        F: FnOnce() -> C;
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<E> OptionExt for Option<E>
where
    E: Error,
{
    type Error = E;

    #[allow(clippy::manual_map)] // Messes with #[track_caller]
    #[track_caller]
    #[inline]
    fn capture<E2, K>(error_kind: K, option: Option<E2>) -> Self
    where
        E2: Context,
        K: ErrorKind + Into<<Self::Error as Error>::Kind>,
        Self::Error: TrivialError,
    {
        match option {
            Some(err) => Some(Self::Error::new(error_kind.into()).with_context(|| err)),
            None => None,
        }
    }

    #[allow(clippy::manual_map)] // Messes with #[track_caller]
    #[track_caller]
    #[inline]
    fn convert<E2, K>(error_kind: K, option: Option<E2>) -> Self
    where
        E2: Context,
        K: ErrorKind + Into<<Self::Error as Error>::Kind>,
        Self::Error: TrivialError,
    {
        match option {
            Some(_) => Some(Self::Error::new(error_kind.into())),
            None => None,
        }
    }

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
