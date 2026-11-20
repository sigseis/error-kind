// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use error_kind::{ErrorDomain, ErrorKind};

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, ErrorKind, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum FooErrorKind {
    Foo1,
    Foo2,
}

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, ErrorKind, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum BarErrorKind {
    Bar1,
    Bar2,
}

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, ErrorKind, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum BazErrorKind {
    Baz1,
    Baz2,
}

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, ErrorDomain, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum FooBarErrorDomain {
    Foo(FooErrorKind),
    Bar(BarErrorKind),
}

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, ErrorDomain, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum FooBazErrorDomain {
    Foo(FooErrorKind),
    #[error("custom error text: {0}")]
    Baz(BazErrorKind),
}

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, ErrorDomain, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum BarBazErrorDomain {
    Bar(BarErrorKind),
    #[error("custom error text: {0}")]
    Baz(BazErrorKind),
}

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, ErrorDomain, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[error(domains(FooBarErrorDomain, FooBazErrorDomain))]
#[error(domains(BarBazErrorDomain))]
enum FooBarBazErrorDomain {
    Foo(FooErrorKind),
    #[error("custom error text")]
    Bar(BarErrorKind),
    #[error("custom error text: {0}")]
    Baz(BazErrorKind),
}

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
#[test]
fn display() {
    assert_eq!(
        FooBarBazErrorDomain::Foo(FooErrorKind::Foo1).to_string(),
        "foo: foo1",
    );
    assert_eq!(
        FooBarBazErrorDomain::Foo(FooErrorKind::Foo2).to_string(),
        "foo: foo2",
    );
    assert_eq!(
        FooBarBazErrorDomain::Bar(BarErrorKind::Bar1).to_string(),
        "custom error text",
    );
    assert_eq!(
        FooBarBazErrorDomain::Bar(BarErrorKind::Bar2).to_string(),
        "custom error text",
    );
    assert_eq!(
        FooBarBazErrorDomain::Baz(BazErrorKind::Baz1).to_string(),
        "custom error text: baz1",
    );
    assert_eq!(
        FooBarBazErrorDomain::Baz(BazErrorKind::Baz2).to_string(),
        "custom error text: baz2",
    );
}

// -----------------------------------------------------------------------------
#[test]
fn from() {
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Relating to: FooErrorKind
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_eq!(
        FooBarErrorDomain::from(FooErrorKind::Foo1),
        FooBarErrorDomain::Foo(FooErrorKind::Foo1),
    );
    assert_eq!(
        FooBarErrorDomain::from(FooErrorKind::Foo2),
        FooBarErrorDomain::Foo(FooErrorKind::Foo2),
    );
    assert_eq!(
        FooBazErrorDomain::from(FooErrorKind::Foo1),
        FooBazErrorDomain::Foo(FooErrorKind::Foo1),
    );
    assert_eq!(
        FooBazErrorDomain::from(FooErrorKind::Foo2),
        FooBazErrorDomain::Foo(FooErrorKind::Foo2),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(FooErrorKind::Foo1),
        FooBarBazErrorDomain::Foo(FooErrorKind::Foo1),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(FooErrorKind::Foo2),
        FooBarBazErrorDomain::Foo(FooErrorKind::Foo2),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(FooBarErrorDomain::Foo(FooErrorKind::Foo1)),
        FooBarBazErrorDomain::Foo(FooErrorKind::Foo1),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(FooBarErrorDomain::Foo(FooErrorKind::Foo2)),
        FooBarBazErrorDomain::Foo(FooErrorKind::Foo2),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(FooBazErrorDomain::Foo(FooErrorKind::Foo1)),
        FooBarBazErrorDomain::Foo(FooErrorKind::Foo1),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(FooBazErrorDomain::Foo(FooErrorKind::Foo2)),
        FooBarBazErrorDomain::Foo(FooErrorKind::Foo2),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Relating to: BarErrorKind
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_eq!(
        FooBarErrorDomain::from(BarErrorKind::Bar1),
        FooBarErrorDomain::Bar(BarErrorKind::Bar1),
    );
    assert_eq!(
        FooBarErrorDomain::from(BarErrorKind::Bar2),
        FooBarErrorDomain::Bar(BarErrorKind::Bar2),
    );
    assert_eq!(
        BarBazErrorDomain::from(BarErrorKind::Bar1),
        BarBazErrorDomain::Bar(BarErrorKind::Bar1),
    );
    assert_eq!(
        BarBazErrorDomain::from(BarErrorKind::Bar2),
        BarBazErrorDomain::Bar(BarErrorKind::Bar2),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(BarErrorKind::Bar1),
        FooBarBazErrorDomain::Bar(BarErrorKind::Bar1),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(BarErrorKind::Bar2),
        FooBarBazErrorDomain::Bar(BarErrorKind::Bar2),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(FooBarErrorDomain::Bar(BarErrorKind::Bar1)),
        FooBarBazErrorDomain::Bar(BarErrorKind::Bar1),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(FooBarErrorDomain::Bar(BarErrorKind::Bar2)),
        FooBarBazErrorDomain::Bar(BarErrorKind::Bar2),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(BarBazErrorDomain::Bar(BarErrorKind::Bar1)),
        FooBarBazErrorDomain::Bar(BarErrorKind::Bar1),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(BarBazErrorDomain::Bar(BarErrorKind::Bar2)),
        FooBarBazErrorDomain::Bar(BarErrorKind::Bar2),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Relating to: BazErrorKind
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_eq!(
        FooBazErrorDomain::from(BazErrorKind::Baz1),
        FooBazErrorDomain::Baz(BazErrorKind::Baz1),
    );
    assert_eq!(
        FooBazErrorDomain::from(BazErrorKind::Baz2),
        FooBazErrorDomain::Baz(BazErrorKind::Baz2),
    );
    assert_eq!(
        BarBazErrorDomain::from(BazErrorKind::Baz1),
        BarBazErrorDomain::Baz(BazErrorKind::Baz1),
    );
    assert_eq!(
        BarBazErrorDomain::from(BazErrorKind::Baz2),
        BarBazErrorDomain::Baz(BazErrorKind::Baz2),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(BazErrorKind::Baz1),
        FooBarBazErrorDomain::Baz(BazErrorKind::Baz1),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(BazErrorKind::Baz2),
        FooBarBazErrorDomain::Baz(BazErrorKind::Baz2),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(FooBazErrorDomain::Baz(BazErrorKind::Baz1)),
        FooBarBazErrorDomain::Baz(BazErrorKind::Baz1),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(FooBazErrorDomain::Baz(BazErrorKind::Baz2)),
        FooBarBazErrorDomain::Baz(BazErrorKind::Baz2),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(BarBazErrorDomain::Baz(BazErrorKind::Baz1)),
        FooBarBazErrorDomain::Baz(BazErrorKind::Baz1),
    );
    assert_eq!(
        FooBarBazErrorDomain::from(BarBazErrorDomain::Baz(BazErrorKind::Baz2)),
        FooBarBazErrorDomain::Baz(BazErrorKind::Baz2),
    );
}

// -----------------------------------------------------------------------------
#[test]
fn is_not() {
    fn implements_trait<A: error_kind::details::IsNot<B>, B: error_kind::details::IsNot<A>>() {}
    implements_trait::<BarBazErrorDomain, BarErrorKind>();
    implements_trait::<BarBazErrorDomain, BazErrorKind>();
    implements_trait::<FooBarBazErrorDomain, BarBazErrorDomain>();
    implements_trait::<FooBarBazErrorDomain, BarErrorKind>();
    implements_trait::<FooBarBazErrorDomain, BazErrorKind>();
    implements_trait::<FooBarBazErrorDomain, FooBarErrorDomain>();
    implements_trait::<FooBarBazErrorDomain, FooBazErrorDomain>();
    implements_trait::<FooBarBazErrorDomain, FooErrorKind>();
    implements_trait::<FooBarErrorDomain, BarErrorKind>();
    implements_trait::<FooBarErrorDomain, FooErrorKind>();
    implements_trait::<FooBazErrorDomain, BazErrorKind>();
    implements_trait::<FooBazErrorDomain, FooErrorKind>();
}

// -----------------------------------------------------------------------------
#[test]
fn match_into() {
    fn implements_trait<From: error_kind::details::MatchInto<Into>, Into>() {}
    implements_trait::<BarBazErrorDomain, FooBarBazErrorDomain>();
    implements_trait::<FooBarErrorDomain, FooBarBazErrorDomain>();
    implements_trait::<FooBazErrorDomain, FooBarBazErrorDomain>();
}
