// =============================================================================
// TRAITS
// =============================================================================

/// Match and convert the fields of an enum.
///
/// This is a generic way to use the trait system to implement a match, followed
/// by a type conversion. That way you don't need to know the variants at the
/// callsite, only that you *can* convert from all possible field values.
///
/// # Detailed Explanation
///
/// At first glance, the `MatchInto` trait can feel a lot like a duplicate of
/// the `Into` trait. But there's two things that set it apart.
///
/// 1. We don't want to compete in the same trait space as `Into`.
/// 2. We want to implement from the perspective of `*Into`.
///
/// When we are creating an enum, we obviously know all the variants of that
/// enum. The same is true about when we're deriving traits for an enum's
/// implementation.
///
/// So we can name our own arms, but we cannot name the target arms.
///
/// ```compile_fail
/// pub enum SubsetEnum {
///     Bool(bool),
///     Unsigned(u32),
/// }
///
/// // When deriving, we know enough to write the following trait impl.
/// impl MatchInto<_> for SubsetEnum {
///     fn match_into(self) -> _ {
///         match self {
///             Self::Bool(value) => _,
///             Self::Unsigned(value) => _,
///         }
///     }
/// }
/// ```
///
/// But if our types are defined in such a way, where the variant itself doesn't
/// matter, and only the *fields* matter - we can abstract the match away and
/// process the conversion over the field data.
///
/// ```
/// # use error_kind::details::MatchInto;
/// pub enum SubsetEnum {
///     Bool(bool),
///     Unsigned(u32),
/// }
///
/// // When deriving, we know enough to write the following trait impl.
/// impl<T> MatchInto<T> for SubsetEnum
/// where
///     T: From<bool> + From<u32>,
/// {
///     fn match_into(self) -> T {
///         match self {
///             Self::Bool(value) => From::from(value),
///             Self::Unsigned(value) => From::from(value),
///         }
///     }
/// }
/// ```
///
/// This, it turns out, allows us to have enough information to implement `From`
/// to convert from another structured type, even if we can't name the variants.
///
/// ```
/// # use error_kind::details::MatchInto;
/// # pub enum SubsetEnum { Bool(bool), Unsigned(u32), }
/// # impl<T: From<bool> + From<u32>> MatchInto<T> for SubsetEnum {
/// #     fn match_into(self) -> T {
/// #         match self {
/// #             Self::Bool(value) => From::from(value),
/// #             Self::Unsigned(value) => From::from(value),
/// #         }
/// #     }
/// # }
/// pub enum SupersetEnum {
///     Bool(bool),
///     Char(char),
///     Unsigned(u32),
/// }
///
/// // When deriving, we only need to know the *name* of the other types.
/// impl From<SubsetEnum> for SupersetEnum {
///     fn from(value: SubsetEnum) -> Self {
///         value.match_into()
///     }
/// }
///
/// // Because we can say how we can convert from each value.
/// impl From<bool> for SupersetEnum {
///     fn from(value: bool) -> Self {
///         Self::Bool(value)
///     }
/// }
/// impl From<char> for SupersetEnum {
///     fn from(value: char) -> Self {
///         Self::Char(value)
///     }
/// }
/// impl From<u32> for SupersetEnum {
///     fn from(value: u32) -> Self {
///         Self::Unsigned(value)
///     }
/// }
/// ```
///
/// This is what allows us to implement conversion between different
/// [`ErrorDomain`]s, without having to know the variant names of the subset
/// domains.
///
/// *We only need to be told that a subset domain exists.*
///
/// ```
/// # use error_kind::{ErrorDomain, ErrorKind};
/// # #[derive(Copy, Clone, Debug, ErrorKind)]
/// # pub enum ErrorKindA {
/// #     Variant,
/// # }
/// # #[derive(Copy, Clone, Debug, ErrorKind)]
/// # pub enum ErrorKindB {
/// #     Variant,
/// # }
/// # #[derive(Copy, Clone, Debug, ErrorKind)]
/// # pub enum ErrorKindC {
/// #     Variant,
/// # }
/// #[derive(Copy, Clone, Debug, ErrorDomain)]
/// pub enum ErrorDomainA {
///     A(ErrorKindA),
///     B(ErrorKindB),
/// }
///
/// #[derive(Copy, Clone, Debug, ErrorDomain)]
/// #[error(domains(ErrorDomainA))] // <- Inform about subset domain.
/// pub enum ErrorDomainB {
///     A(ErrorKindA),
///     B(ErrorKindB),
///     C(ErrorKindC),
/// }
///
/// #[derive(Clone, Debug, error_kind::Error)]
/// #[error(domains(ErrorDomainA, ErrorDomainB))] // <- Register all domains.
/// pub struct Error<K: ErrorKind>(K);
///
/// fn emit_result_domain_a() -> Result<(), Error<ErrorDomainA>> {
///     # unimplemented!()
///     // ...
/// }
///
/// // Thanks to `MatchInto`, we can do this!
/// fn emit_result_domain_b() -> Result<(), Error<ErrorDomainB>> {
///     emit_result_domain_a()?; // Converts to `ErrorDomainB`!
///     Ok(())
/// }
/// ```
///
/// [`ErrorDomain`]: crate::ErrorDomain
pub trait MatchInto<T>: Sized {
    /// Convert from `Self` into `T` via a match + conversion.
    fn match_into(self) -> T;
}
