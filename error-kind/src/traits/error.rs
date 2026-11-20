// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::{ErrorContext, ErrorKind};

// =============================================================================
// TRAITS
// =============================================================================

/// An error that contains an associated [`ErrorKind`].
///
/// Error types are expected to be fairly simple structs, usually containing
/// only a single field.
///
/// ```
/// use error_kind::{Error, ErrorKind};
///
/// #[derive(Clone, Copy, Debug, ErrorKind)]
/// enum MyErrorKind {
///     Variant,
/// }
///
/// #[derive(Debug, Error)]
/// struct MyError(MyErrorKind);
/// ```
///
/// If you plan on having many different kinds of errors within a crate, you may
/// wish to make your type generic over the kind of error.
///
/// ```
/// use error_kind::{Error, ErrorKind};
///
/// #[derive(Debug, Error)]
/// struct MyError<K: ErrorKind>(K);
/// ```
///
/// Your error kind can also be wrapped in [`envelopes`] to collect additional
/// error information.
///
/// [`envelopes`]: crate::envelopes
///
/// # Traits Implemented
///
/// The derive macro helps us out a lot here, implementing the following traits:
///
/// - [`Display`]
/// - [`ErrorContext`]
/// - [`Error`] (both `core` and `error_kind`)
/// - [`From`] (via `#[error(domains(...))]`)
/// - [`TrivialError`]
///
/// [`Display`]: core::fmt::Display
/// [`TrivialError`]: crate::TrivialError
///
/// You can disable any of these by using the `#[error(disable(...))]` attribute
/// (e.g. `#[error(disable(From, Display))]`).
///
/// You can force-hide certain trait implementations from documentation by using
/// the `#[error(hidden(...))]` attribute (e.g.
/// `#[error(hidden(TrivialError))]`).
///
/// ```rust
/// # use error_kind::{Error, ErrorKind};
/// # #[derive(Clone, Copy, Debug, ErrorKind)]
/// # enum MyErrorKind { Variant }
///
/// #[derive(Debug, Error)]
/// #[error(disable(Display, TrivialError))]
/// #[error(hidden(core::error::Error))]
/// struct NoDisplayExample(MyErrorKind);
///
/// // Though, now you have to implement `Display` yourself.
/// // This is required since `Display` is a supertrait of `Error`.
/// impl std::fmt::Display for NoDisplayExample {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         # unimplemented!()
///         // ...
///     }
/// }
/// ```
///
/// # Error Domains
///
/// If your crate defines any [`ErrorDomain`]s that you would like your error to
/// be able to convert between, register them with `#[error(domains(...))]`.
///
/// [`ErrorDomain`]: crate::ErrorDomain
///
/// **This allows for...**
///
/// - Conversion from `MyError<MyErrorKind>` to `MyError<_>` for any
///   registered [`ErrorDomain`] which contains `MyErrorKind`.
/// - Conversion from `MyError<MyErrorDomain>` to `MyError<_>` for any
///   registered [`ErrorDomain`] which contains a superset of all [`ErrorKind`]s
///   present in `MyErrorDomain`.
///
/// ```rust
/// # use error_kind::{Error, ErrorDomain, ErrorKind};
/// # #[derive(Clone, Copy, Debug, ErrorKind, PartialEq)]
/// # enum MyErrorKind1 { Variant }
/// # #[derive(Clone, Copy, Debug, ErrorKind, PartialEq)]
/// # enum MyErrorKind2 { Variant }
/// # #[derive(Clone, Copy, Debug, ErrorKind, PartialEq)]
/// # enum MyErrorKind3 { Variant }
/// #[derive(Clone, Copy, Debug, ErrorDomain, PartialEq)]
/// enum MyErrorDomain1 {
///     Kind1(MyErrorKind1),
///     Kind2(MyErrorKind2),
///     // ...
/// }
///
/// #[derive(Clone, Copy, Debug, ErrorDomain, PartialEq)]
/// #[error(domains(MyErrorDomain1))] // <- Suggest a subset domain
/// enum MyErrorDomain2 {
///     Kind1(MyErrorKind1),
///     Kind2(MyErrorKind2),
///     Kind3(MyErrorKind3),
///     // ...
/// }
///
/// #[derive(Debug, Error, Eq, PartialEq)]
/// #[error(domains(MyErrorDomain1, MyErrorDomain2))] // Because of this...
/// struct MyError<K: ErrorKind>(K);
///
/// // ... these conversions work as expected!
/// assert_eq!(
///     MyError(MyErrorDomain1::Kind1(MyErrorKind1::Variant)),
///     From::from(MyError(MyErrorKind1::Variant)),
/// );
/// assert_eq!(
///     MyError(MyErrorDomain2::Kind1(MyErrorKind1::Variant)),
///     From::from(MyError(MyErrorKind1::Variant)),
/// );
///
/// // ... and perhaps surprisingly, this also works!
/// assert_eq!(
///     MyError(MyErrorDomain2::Kind1(MyErrorKind1::Variant)),
///     From::from(MyError(MyErrorDomain1::Kind1(MyErrorKind1::Variant))),
/// );
/// ```
///
/// > **Learn More:** While you don't need to understand how this works, if you
/// > are curious to learn, see the [`IsNot`] and [`MatchInto`] traits.
///
/// [`IsNot`]: crate::details::IsNot
/// [`MatchInto`]: crate::details::MatchInto
///
/// # Trivial Errors
///
/// The [`TrivialError`] trait is used to power the [`err!`] macro.
///
/// If the error type only contains one field (the error kind), it's said to be
/// trivial. In these cases, we can easily say how to properly construct it!
///
/// ```
/// use error_kind::{Error, ErrorKind, TrivialError};
///
/// #[derive(Clone, Copy, Debug, ErrorKind)]
/// enum MyErrorKind {
///     Variant,
/// }
///
/// #[derive(Debug, Error)]
/// struct MyError(MyErrorKind);
///
/// // Allowed via the `TrivialError` trait.
/// let _ = MyError::new(MyErrorKind::Variant);
/// ```
///
/// However, if more than one field is present, the derive macro is not certain
/// that taking the `Default` for the other values is the right thing to do.
///
/// As such, it will simply avoid implementing this trait.
///
/// ```compile_fail
/// use error_kind::{Error, ErrorKind, TrivialError};
///
/// #[derive(Clone, Copy, Debug, ErrorKind)]
/// enum MyErrorKind {
///     Variant,
/// }
///
/// #[derive(Debug, Error)]
/// struct MyError(#[error_kind]MyErrorKind, u32);
///
/// // Not allowed, because of multiple fields. :(
/// let _ = MyError::new(MyErrorKind::Variant);
/// ```
///
/// In those cases, if you know better, you can implement [`TrivialError`]
/// manually.
///
/// [`err!`]: crate::err
///
/// # Recommendation: Provide an `error_kind` Function
///
/// Though users can depend on this trait to acquire the underlying error kind,
/// it's more ergonomic to simply provide a function on the error type itself.
///
/// **The `Error` derive macro *will not* implement this for you!**
///
/// ```
/// use error_kind::{Error, ErrorKind};
///
/// #[derive(Copy, Clone, Debug, ErrorKind)]
/// pub enum MyErrorKind {
///     Error1,
/// }
///
/// #[derive(Debug, Error)]
/// pub struct MyError {
///     error_kind: MyErrorKind,
/// }
///
/// impl MyError {
///     // We have to implement this manually, if we want it.
///     pub fn error_kind(&self) -> MyErrorKind {
///         self.error_kind
///     }
/// }
/// ```
///
/// # Examples
///
/// This type supports named and unnamed type constructions.
///
/// ```rust
/// # use error_kind::{Error, ErrorKind};
/// # #[derive(Clone, Copy, Debug, ErrorKind)]
/// # enum MyErrorKind { Variant }
/// #[derive(Debug, Error)]
/// struct NamedExample {
///     error_kind: MyErrorKind,
/// }
///
/// #[derive(Debug, Error)]
/// struct UnnamedExample(MyErrorKind);
/// ```
///
/// If multiple fields are present, one field must be marked as the field which
/// contains our `ErrorKind` (using `#[error_kind]`).
///
/// ```rust
/// # use error_kind::{Error, ErrorKind};
/// # #[derive(Clone, Copy, Debug, ErrorKind)]
/// # enum MyErrorKind { Variant }
/// #[derive(Debug, Error)]
/// struct MyError(u32, #[error_kind]MyErrorKind, i16);
/// ```
///
/// Your error type is allowed to be generic over the `ErrorKind`. It's highly
/// recommended that you add a bounds to the type parameter, however.
///
/// ```rust
/// # use error_kind::{Error, ErrorKind};
/// # #[derive(Clone, Copy, Debug, ErrorKind)]
/// # enum MyErrorKind { Variant }
/// #[derive(Debug, Error)]
/// struct MyError<K: ErrorKind> {
///     field_a: u32,
///     #[error_kind]
///     field_b: K,
///     field_c: i16,
/// }
/// ```
pub trait Error: core::error::Error + ErrorContext {
    /// The `ErrorKind` that this error contains.
    type Kind: ErrorKind;

    /// Returns the `ErrorKind` contained within this error.
    fn error_kind(&self) -> Self::Kind;
}
