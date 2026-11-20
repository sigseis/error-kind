// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
use std_alloc::format;

// =============================================================================
// HELPER TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq)]
enum TestErrorKind {
    Variant,
}

// -----------------------------------------------------------------------------
impl ErrorKind for TestErrorKind {}

// -----------------------------------------------------------------------------
impl core::fmt::Display for TestErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("variant")
    }
}

// =============================================================================
// MACROS
// =============================================================================

// -----------------------------------------------------------------------------
macro_rules! assert_specific_err {
    ($format:literal, $error:expr, $expected:expr) => {
        assert_eq!(
            format!($format, Located::<TestErrorKind>::from($error)),
            $expected,
        )
    };
    ($format:literal, $wrapper:ident, $error_kind:expr, $description:literal; $($expected:tt)+) => {
        assert_specific_err!(
            $format,
            $wrapper::with_description($error_kind, $description),
            format!($($expected)*)
        )
    };
    ($format:literal, $wrapper:ident, $error_kind:expr; $($expected:tt)+) => {
        assert_specific_err!(
            $format,
            $wrapper::new($error_kind),
            format!($($expected)*)
        )
    };
}

// -----------------------------------------------------------------------------
macro_rules! assert_alt_err {
    ($($tt:tt)+) => {
        assert_specific_err!("{:#}", $($tt)*)
    };
}

// -----------------------------------------------------------------------------
macro_rules! assert_reg_err {
    ($($tt:tt)+) => {
        assert_specific_err!("{}", $($tt)*)
    };
}

// -----------------------------------------------------------------------------
macro_rules! assert_err {
    ($($tt:tt)+) => {{
        assert_alt_err!($($tt)*);
        assert_reg_err!($($tt)*);
    }};
}

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
#[test]
fn from_described() {
    use crate::envelopes::Described;
    assert_err!(
        Described,
        TestErrorKind::Variant;
        "variant",
    );
    assert_err!(
        Described,
        TestErrorKind::Variant,
        "error description";
        "variant",
    );
}

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
#[test]
fn from_diagnostic() {
    use crate::envelopes::Diagnostic;
    assert_alt_err!(
        Diagnostic,
        TestErrorKind::Variant;
        "variant\n  at {file}:{line}:5",
        file = file!(),
        line = line!() - 5,
    );
    assert_reg_err!(
        Diagnostic,
        TestErrorKind::Variant;
        "variant"
    );
    assert_alt_err!(
        Diagnostic,
        TestErrorKind::Variant,
        "error description";
        "variant\n  at {file}:{line}:5",
        file = file!(),
        line = line!() - 6,
    );
    assert_reg_err!(
        Diagnostic,
        TestErrorKind::Variant,
        "error description";
        "variant"
    );
}

// -----------------------------------------------------------------------------
#[test]
fn from_error_kind() {
    assert_err!(
        TestErrorKind,
        TestErrorKind::Variant;
        "variant",
    );
    assert_err!(
        TestErrorKind,
        TestErrorKind::Variant,
        "error description";
        "variant",
    );
}
