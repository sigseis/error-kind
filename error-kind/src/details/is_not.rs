// =============================================================================
// TRAITS
// =============================================================================

/// Denotes that two types are distinct from one-another.
///
/// Useful for adding a generic negative bounds on a fully-typed trait impl.
///
/// ```
/// # use error_kind::details::IsNot;
/// # struct Wrapper<T>(T);
/// # struct MyType;
/// impl<T> From<Wrapper<T>> for Wrapper<MyType>
/// where
///     T: IsNot<MyType>,
///     MyType: From<T>,
/// {
///     fn from(value: Wrapper<T>) -> Self {
///         Wrapper(From::from(value.0))
///     }
/// }
/// ```
///
/// `IsNot<T>` can only restrict trait bounds when `Self` is *fully-defined*
/// (non-generic).
///
/// This trait **will not help** if a generic type parameter is used in `Self`.
///
/// # Detailed Explanation
///
/// Trait bounds lack the ability to restrict types based on negative trait
/// bounds (`T: !Trait`). Likewise, trait bounds also lack the ability to
/// restrict types to only those not matching a given type (`T != Type`).
///
/// This leads to issues with blanket generic implementations. The most common
/// of which is the `From` trait. The standard library provides the following
/// default implementation.
///
/// ```
/// # trait From<T> { fn from(v: T) -> Self; }
/// impl<T> From<T> for T {
///     fn from(value: T) -> Self {
///         value
///     }
/// }
/// ```
///
/// This usually isn't an issue is because we implement over specific types. The
/// compiler is able to deduce that the two implementations don't conflict.
///
/// ```
/// # struct TypeA;
/// # struct TypeB;
/// // This is okay, since we are certain `TypeA` != `TypeB`.
/// // The implementation over `From<T> for T` is disjoint.
/// impl From<TypeA> for TypeB {
///     fn from(value: TypeA) -> Self {
///         # unimplemented!()
///         // ...
///     }
/// }
/// ```
///
/// Where it becomes annoying is when we have wrapping types. We often want to
/// be able to convert the inside of a wrapping type if the inner type supports
/// `From` conversion.
///
/// **But we cannot do this generically!**
///
/// ```compile_fail
/// # struct TypeA;
/// # struct TypeB;
/// # struct Wrapper<T>(T);
/// # impl From<TypeA> for TypeB { fn from(value: TypeA) -> Self { unimplemented!() } }
/// // This fails to compile...
/// impl<T: From<TypeA>> From<Wrapper<TypeA>> for Wrapper<T> {
///     fn from(value: Wrapper<TypeA>) -> Self {
///         Wrapper(From::from(value.0))
///     }
/// }
/// ```
///
/// ```compile_fail
/// # struct TypeA;
/// # struct TypeB;
/// # struct Wrapper<T>(T);
/// # impl From<TypeA> for TypeB { fn from(value: TypeA) -> Self { unimplemented!() } }
/// // This fails to compile...
/// impl<T> From<Wrapper<T>> for Wrapper<TypeA>
/// where
///     TypeA: From<T>,
/// {
///     fn from(value: Wrapper<T>) -> Self {
///         Wrapper(From::from(value.0))
///     }
/// }
/// ```
///
/// If we had defined the `Wrapper` type, then we can get around this in a
/// similar manner to `TypeA` and `TypeB`, by simply not using generics.
///
/// ```
/// # struct TypeA;
/// # struct TypeB;
/// # struct Wrapper<T>(T);
/// # impl From<TypeA> for TypeB { fn from(value: TypeA) -> Self { unimplemented!() } }
/// # impl From<TypeB> for TypeA { fn from(value: TypeB) -> Self { unimplemented!() } }
/// // These work because they're not generic *and* local types.
/// impl From<Wrapper<TypeA>> for Wrapper<TypeB> {
///     fn from(value: Wrapper<TypeA>) -> Self {
///         Wrapper(From::from(value.0))
///     }
/// }
/// impl From<Wrapper<TypeB>> for Wrapper<TypeA> {
///     fn from(value: Wrapper<TypeB>) -> Self {
///         Wrapper(From::from(value.0))
///     }
/// }
/// ```
///
/// But you cannot do this with types and traits that you haven't defined
/// locally within your crate.
///
/// ```compile_fail
/// # struct TypeA;
/// # struct TypeB;
/// # impl From<TypeA> for TypeB { fn from(value: TypeA) -> Self { unimplemented!() } }
/// # impl From<TypeB> for TypeA { fn from(value: TypeB) -> Self { unimplemented!() } }
/// // This doesn't work because `Option` and `From` are foreign.
/// impl From<Option<TypeA>> for Option<TypeB> {
///     fn from(value: Option<TypeA>) -> Self {
///         value.map(From::from)
///     }
/// }
/// ```
///
/// All this is an issue for this crate. The main way you ergonomically deal
/// with error handling is to allow for errors to convert between one-another.
///
/// We have exactly this issue where we have a wrapping type ([`Error`]), which
/// contains an underlying type ([`ErrorKind`]), and we need to support generic
/// conversion for the wrapping type.
///
/// At first glance, this seems to not work, even with the `IsNot` trait.
///
/// ```compile_fail
/// # struct TypeA;
/// # struct TypeB;
/// # struct Wrapper<T>(T);
/// # use error_kind::details::IsNot;
/// # impl From<TypeA> for TypeB { fn from(value: TypeA) -> Self { unimplemented!() } }
/// // Say that the types are distinct from each other...
/// impl IsNot<TypeA> for TypeB {}
/// impl IsNot<TypeB> for TypeA {}
///
/// // We still conflict with `From<T> for T`... :(
/// impl<T, U> From<Wrapper<T>> for Wrapper<U>
/// where
///     T: IsNot<U>,
///     U: IsNot<T> + From<T>,
/// {
///     fn from(value: Wrapper<T>) -> Self {
///         Wrapper(From::from(value.0))
///     }
/// }
/// ```
///
/// However, if the `Self` type is non-generic, the trait solver can perform
/// this restrictive check on the input bounds. Since you control the
/// implementation of `IsNot` over your local types, nobody can come along and
/// implement this invalidly.
///
/// **So, surprisingly, as long as `Self` is non-generic, this works!**
///
/// ```
/// # struct TypeA;
/// # struct TypeB;
/// # struct Wrapper<T>(T);
/// # use error_kind::details::IsNot;
/// # impl From<TypeA> for TypeB { fn from(value: TypeA) -> Self { unimplemented!() } }
/// // Say that the types are distinct from each other...
/// impl IsNot<TypeA> for TypeB {}
/// impl IsNot<TypeB> for TypeA {}
///
/// // Oh, look! If `Self` is non-generic, this works!!
/// impl<T> From<Wrapper<T>> for Wrapper<TypeA>
/// where
///     T: IsNot<TypeA>,
///     TypeA: From<T>,
/// {
///     fn from(value: Wrapper<T>) -> Self {
///         Wrapper(From::from(value.0))
///     }
/// }
///
/// // We can say the other way around too - for `TypeB`!
/// impl<T> From<Wrapper<T>> for Wrapper<TypeB>
/// where
///     T: IsNot<TypeB>,
///     TypeB: From<T>,
/// {
///     fn from(value: Wrapper<T>) -> Self {
///         Wrapper(From::from(value.0))
///     }
/// }
/// ```
///
/// **This is what allows the following code to compile.**
///
/// - The [`Error`] type is defined in the dependant's crate (it's local).
/// - [`ErrorDomain`] defines `IsNot` for all variant field types.
/// - We only have to name the domain at the `Error` site to register it.
///
/// ```
/// use error_kind::ErrorKind;
///
/// #[derive(Copy, Clone, Debug, ErrorKind)]
/// pub enum ErrorKindA {
///     Variant,
/// }
///
/// #[derive(Copy, Clone, Debug, ErrorKind)]
/// pub enum ErrorKindB {
///     Variant,
/// }
///
/// #[derive(Copy, Clone, Debug, error_kind::ErrorDomain)]
/// pub enum ErrorDomain {
///     A(ErrorKindA),
///     B(ErrorKindB),
/// }
///
/// #[derive(Clone, Debug, error_kind::Error)]
/// #[error(domains(ErrorDomain))]
/// pub struct Error<K: ErrorKind>(K);
///
/// fn emit_result_a() -> Result<(), Error<ErrorKindA>> {
///     # unimplemented!()
///     // ...
/// }
///
/// fn emit_result_b() -> Result<(), Error<ErrorKindB>> {
///     # unimplemented!()
///     // ...
/// }
///
/// // Thanks to `IsNot`, we can do this:
/// fn emit_result_domain() -> Result<(), Error<ErrorDomain>> {
///     emit_result_a()?; // Converts to `ErrorDomain`!
///     emit_result_b()?; // Converts to `ErrorDomain`!
///     Ok(())
/// }
/// ```
///
/// [`ErrorDomain`]: crate::ErrorDomain
/// [`ErrorKind`]: crate::ErrorKind
/// [`Error`]: crate::Error
/// [`Try`]: core::ops::Try
pub trait IsNot<T> {}
