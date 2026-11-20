// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::ErrorKind;

// =============================================================================
// TRAITS
// =============================================================================

/// A marker trait for a type that denotes a domain of possible error kinds.
///
/// These types are expected to be very simple enums, with the following rules:
///
/// - Each field must contain a single unnamed field.
/// - The unnamed field must contain an [`ErrorKind`] type.
/// - Each type may only be used once within the enum.
///
/// # Display Formatting
///
/// The derive macro for this trait will automatically implement `Display` for
/// you. The default process is to break the variant name into words, lowercase
/// it, and then separate the words by spaces.
///
/// Then, the error domain will print alongside its value: "{domain}: {value}".
///
/// ```rust
/// # use error_kind::{ErrorDomain, ErrorKind};
/// # #[derive(Clone, Copy, Debug, ErrorKind)]
/// # enum IoErrorKind { Variant }
/// # #[derive(Clone, Copy, Debug, ErrorKind)]
/// # enum ParseErrorKind { Variant }
/// #[derive(Clone, Copy, Debug, ErrorDomain)]
/// enum ParseErrorDomain {
///     Io(IoErrorKind),        // Prints "io: {0}"
///     Parse(ParseErrorKind),  // Prints "parse: {0}"
///     // ...
/// }
/// ```
///
/// If you don't like a particular display format, you can change it with the
/// `#[error(...)]` attribute on a variant.
///
/// ```rust
/// # use error_kind::{ErrorDomain, ErrorKind};
/// # #[derive(Clone, Copy, Debug, ErrorKind)]
/// # enum IoErrorKind { Variant }
/// # #[derive(Clone, Copy, Debug, ErrorKind)]
/// # enum ParseErrorKind { Variant }
/// #[derive(Clone, Copy, Debug, ErrorDomain)]
/// enum ParseErrorDomain {
///     Io(IoErrorKind),        // Prints "io: {0}"
///     #[error("custom: {0}")]
///     Parse(ParseErrorKind),  // Prints "custom: {0}"
///     // ...
/// }
/// ```
///
/// The `{0}` will expand to the value in the unnamed field.
///
/// # Purpose
///
/// An `ErrorDomain` allows us to compose multiple different error kinds
/// together.
///
/// This allows us to maintain the following design principles:
///
/// 1. Avoid deeply-nested enums to keep error kinds small and efficient.
/// 2. Ensure that subset type can unambiguously convert into any given superset
///    `ErrorDomain`.
///
/// The second point is the *most important*, because it'll allow for very
/// ergonomic usage of your error types. Thanks to this rule, from conversion
/// for `Error<SubsetDomain>` can be implemented for `Error<SupersetDomain>`.
///
/// ```rust
/// # use error_kind::{ErrorDomain, ErrorKind};
/// #[derive(Clone, Copy, Debug, ErrorKind, PartialEq)]
/// enum IoErrorKind {
///     BadFileDescriptor,
///     FileNotFound,
///     // ...
/// }
///
/// #[derive(Clone, Copy, Debug, ErrorKind, PartialEq)]
/// enum ParseErrorKind {
///     InvalidFormat,
///     UnexpectedEndOfData,
///     // ...
/// }
///
/// #[derive(Clone, Copy, Debug, ErrorDomain, PartialEq)]
/// enum ParseErrorDomain {
///     Io(IoErrorKind),
///     Parse(ParseErrorKind),
///     // ...
/// }
///
/// // Thanks to the rules of `ErrorDomain`, this is possible!
/// assert_eq!(
///     ParseErrorDomain::Io(IoErrorKind::FileNotFound),
///     From::from(IoErrorKind::FileNotFound),
/// );
/// ```
///
/// This also works across `ErrorDomain` types as well.
///
/// ```rust
/// # use error_kind::{ErrorDomain, ErrorKind};
/// # #[derive(Clone, Copy, Debug, ErrorKind, PartialEq)]
/// # enum IoErrorKind {
/// #     BadFileDescriptor,
/// #     FileNotFound,
/// #     // ...
/// # }
/// # #[derive(Clone, Copy, Debug, ErrorKind, PartialEq)]
/// # enum ParseErrorKind {
/// #     InvalidFormat,
/// #     UnexpectedEndOfData,
/// #     // ...
/// # }
/// # #[derive(Clone, Copy, Debug, ErrorDomain, PartialEq)]
/// # enum ParseErrorDomain {
/// #     Io(IoErrorKind),
/// #     Parse(ParseErrorKind),
/// #     // ...
/// # }
/// #[derive(Clone, Copy, Debug, ErrorKind, PartialEq)]
/// enum LogicErrorKind {
///     InvalidOperation,
///     TimedOut,
///     // ...
/// }
///
/// #[derive(Clone, Copy, Debug, ErrorDomain, PartialEq)]
/// #[error(domains(ParseErrorDomain))] // <- List subset domains here.
/// enum ProcessingErrorDomain {
///     Io(IoErrorKind),
///     Logic(LogicErrorKind),
///     Parse(ParseErrorKind),
///     // ...
/// }
///
/// // Thanks to the rules of `ErrorDomain`, this is possible!
/// assert_eq!(
///     ProcessingErrorDomain::Io(IoErrorKind::FileNotFound),
///     From::from(ParseErrorDomain::Io(IoErrorKind::FileNotFound)),
/// );
/// ```
///
/// > **Learn More:** While you don't need to understand how this works, if you
/// > are curious to learn, see the [`IsNot`] and [`MatchInto`] traits.
///
/// [`IsNot`]: crate::details::IsNot
/// [`MatchInto`]: crate::details::MatchInto
pub trait ErrorDomain: ErrorKind {}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<D: ErrorDomain> ErrorKind for D {}
