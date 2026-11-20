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
    impl<T, E: super::Error> Sealed for Result<T, E> {}
}

// =============================================================================
// TRAITS
// =============================================================================

/// Extension functions for `Result<T, E>` when `E` is an [`Error`].
pub trait ResultExt: sealed::Sealed {
    /// The error type associated with this result (`Err(_)`).
    type Error: Error;

    /// The result type associated with this result (`Ok(_)`).
    type Result: Sized;

    /// Convert a result from some foreign error, capturing the error context.
    ///
    /// This is equivalent to the following code:
    ///
    /// ```
    /// # use error_kind::{Error, ErrorContext, ErrorKind, ResultExt, TrivialError};
    /// # #[derive(Clone, Copy, Debug, ErrorKind)]
    /// # enum MyErrorKind { Variant }
    /// # #[derive(Clone, Copy, Debug, Error)]
    /// # struct TargetError(MyErrorKind);
    /// # let result = Result::<(), std::io::Error>::Ok(());
    /// # let error_kind = MyErrorKind::Variant;
    /// # let end_result: Result<(), TargetError> =
    /// match result {
    ///     Ok(value) => Ok(value),
    ///     Err(error) => Err(
    ///         TargetError::new(error_kind.into()).with_context(|| error)
    ///     ),
    /// }
    /// # ;
    /// ```
    fn capture<E, K>(error_kind: K, result: Result<Self::Result, E>) -> Self
    where
        E: Context,
        K: ErrorKind + Into<<Self::Error as Error>::Kind>,
        Self::Error: TrivialError;

    /// Convert a result from some foreign error.
    ///
    /// This is equivalent to the following code:
    ///
    /// ```
    /// # use error_kind::{Error, ErrorContext, ErrorKind, ResultExt, TrivialError};
    /// # #[derive(Clone, Copy, Debug, ErrorKind)]
    /// # enum MyErrorKind { Variant }
    /// # #[derive(Clone, Copy, Debug, Error)]
    /// # struct TargetError(MyErrorKind);
    /// # let result = Result::<(), std::io::Error>::Ok(());
    /// # let error_kind = MyErrorKind::Variant;
    /// # let end_result: Result<(), TargetError> =
    /// match result {
    ///     Ok(value) => Ok(value),
    ///     Err(_) => Err(TargetError::new(error_kind.into())),
    /// }
    /// # ;
    /// ```
    fn convert<E, K>(error_kind: K, result: Result<Self::Result, E>) -> Self
    where
        E: Context,
        K: ErrorKind + Into<<Self::Error as Error>::Kind>,
        Self::Error: TrivialError;

    /// Provide human-readable information about why an error occurred.
    ///
    /// This is a wrapper which redirects to [`ErrorContext::with_context`] for
    /// the wrapped `Err(error)`. If the `Result` is `Ok(_)`, this information
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
impl<T, E> ResultExt for Result<T, E>
where
    E: Error,
{
    type Error = E;
    type Result = T;

    #[track_caller]
    #[inline]
    fn capture<E2, K>(error_kind: K, result: Result<Self::Result, E2>) -> Self
    where
        E2: Context,
        K: ErrorKind + Into<<Self::Error as Error>::Kind>,
        Self::Error: TrivialError,
    {
        match result {
            Ok(value) => Ok(value),
            Err(err) => Err(Self::Error::new(error_kind.into()).with_context(|| err)),
        }
    }

    #[track_caller]
    #[inline]
    fn convert<E2, K>(error_kind: K, result: Result<Self::Result, E2>) -> Self
    where
        E2: Context,
        K: ErrorKind + Into<<Self::Error as Error>::Kind>,
        Self::Error: TrivialError,
    {
        match result {
            Ok(value) => Ok(value),
            Err(_) => Err(Self::Error::new(error_kind.into())),
        }
    }

    #[track_caller]
    #[inline]
    fn with_context<C, F>(self, context: F) -> Self
    where
        C: Context,
        F: FnOnce() -> C,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(err.with_context(context)),
        }
    }
}
