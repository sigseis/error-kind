// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use error_kind::err;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
#[test]
fn context() {
    fn implements_trait<T: error_kind::Context>() {}
    implements_trait::<DescribedError>();
    implements_trait::<DescribedErrorAlt>();
    // The implementation for this is validated in `display` tests.
}

// -----------------------------------------------------------------------------
#[test]
fn display() {
    fn with_macro(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1)),
            2 => Err(err!(SimpleErrorKind::Error2)),
            3 => Err(err!(SimpleErrorKind::Error3)),
            _ => Ok(()),
        }
    }
    fn with_macro_trailing_comma(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1,)),
            2 => Err(err!(SimpleErrorKind::Error2,)),
            3 => Err(err!(SimpleErrorKind::Error3,)),
            _ => Ok(()),
        }
    }
    fn without_macro(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => Err(DescribedError::new(SimpleErrorKind::Error1)),
            2 => Err(DescribedError::new(SimpleErrorKind::Error2)),
            3 => Err(DescribedError::new(SimpleErrorKind::Error3)),
            _ => Ok(()),
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // BASIC EQUALITY
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_multi_eq!(
        with_macro(1),
        with_macro_trailing_comma(1),
        without_macro(1),
        Err(DescribedError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(DescribedError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(DescribedError::new(SimpleErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(with_macro(1), "error1");
    assert_err!(with_macro(2), "error2");
    assert_err!(with_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(with_macro(1), "error1");
    assert_alt_err!(with_macro(2), "error2");
    assert_alt_err!(with_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(with_macro_trailing_comma(1), "error1");
    assert_err!(with_macro_trailing_comma(2), "error2");
    assert_err!(with_macro_trailing_comma(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(with_macro_trailing_comma(1), "error1");
    assert_alt_err!(with_macro_trailing_comma(2), "error2");
    assert_alt_err!(with_macro_trailing_comma(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(without_macro(1), "error1");
    assert_err!(without_macro(2), "error2");
    assert_err!(without_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(without_macro(1), "error1");
    assert_alt_err!(without_macro(2), "error2");
    assert_alt_err!(without_macro(3), "error3");
}

// -----------------------------------------------------------------------------
#[test]
fn display_with_description() {
    fn with_macro(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a")),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b")),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c")),
            _ => Ok(()),
        }
    }
    fn with_macro_trailing_comma(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a",)),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b",)),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c",)),
            _ => Ok(()),
        }
    }
    fn without_macro(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => Err(DescribedError::with_description(
                SimpleErrorKind::Error1,
                "example description a",
            )),
            2 => Err(DescribedError::with_description(
                SimpleErrorKind::Error2,
                "example description b",
            )),
            3 => Err(DescribedError::with_description(
                SimpleErrorKind::Error3,
                "example description c",
            )),
            _ => Ok(()),
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // BASIC EQUALITY
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_multi_eq!(
        with_macro(1),
        with_macro_trailing_comma(1),
        without_macro(1),
        Err(DescribedError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(DescribedError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(DescribedError::new(SimpleErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(with_macro(1), "error1: example description a");
    assert_err!(with_macro(2), "error2: example description b");
    assert_err!(with_macro(3), "error3: example description c");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(with_macro(1), "error1: example description a");
    assert_alt_err!(with_macro(2), "error2: example description b");
    assert_alt_err!(with_macro(3), "error3: example description c");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(
        with_macro_trailing_comma(1),
        "error1: example description a"
    );
    assert_err!(
        with_macro_trailing_comma(2),
        "error2: example description b"
    );
    assert_err!(
        with_macro_trailing_comma(3),
        "error3: example description c"
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(
        with_macro_trailing_comma(1),
        "error1: example description a"
    );
    assert_alt_err!(
        with_macro_trailing_comma(2),
        "error2: example description b"
    );
    assert_alt_err!(
        with_macro_trailing_comma(3),
        "error3: example description c"
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(without_macro(1), "error1: example description a");
    assert_err!(without_macro(2), "error2: example description b");
    assert_err!(without_macro(3), "error3: example description c");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(without_macro(1), "error1: example description a");
    assert_alt_err!(without_macro(2), "error2: example description b");
    assert_alt_err!(without_macro(3), "error3: example description c");
}

// -----------------------------------------------------------------------------
// Described errors don't collect error context information.
// -----------------------------------------------------------------------------
#[test]
fn display_with_context() {
    use error_kind::{ErrorContext, error};
    #[error("additional error context")]
    fn with_macro(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a")),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b")),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c")),
            _ => Ok(()),
        }
    }
    #[error("additional error context")]
    fn with_macro_trailing_comma(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a",)),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b",)),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c",)),
            _ => Ok(()),
        }
    }
    fn without_macro(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => Err(DescribedError::with_description(
                SimpleErrorKind::Error1,
                "example description a",
            )
            .with_context(|| "additional error context")),
            2 => Err(DescribedError::with_description(
                SimpleErrorKind::Error2,
                "example description b",
            )
            .with_context(|| "additional error context")),
            3 => Err(DescribedError::with_description(
                SimpleErrorKind::Error3,
                "example description c",
            )
            .with_context(|| "additional error context")),
            _ => Ok(()),
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // BASIC EQUALITY
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_multi_eq!(
        with_macro(1),
        with_macro_trailing_comma(1),
        without_macro(1),
        Err(DescribedError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(DescribedError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(DescribedError::new(SimpleErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(with_macro(1), "error1: example description a");
    assert_err!(with_macro(2), "error2: example description b");
    assert_err!(with_macro(3), "error3: example description c");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(with_macro(1), "error1: example description a");
    assert_alt_err!(with_macro(2), "error2: example description b");
    assert_alt_err!(with_macro(3), "error3: example description c");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(
        with_macro_trailing_comma(1),
        "error1: example description a"
    );
    assert_err!(
        with_macro_trailing_comma(2),
        "error2: example description b"
    );
    assert_err!(
        with_macro_trailing_comma(3),
        "error3: example description c"
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(
        with_macro_trailing_comma(1),
        "error1: example description a"
    );
    assert_alt_err!(
        with_macro_trailing_comma(2),
        "error2: example description b"
    );
    assert_alt_err!(
        with_macro_trailing_comma(3),
        "error3: example description c"
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(without_macro(1), "error1: example description a");
    assert_err!(without_macro(2), "error2: example description b");
    assert_err!(without_macro(3), "error3: example description c");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(without_macro(1), "error1: example description a");
    assert_alt_err!(without_macro(2), "error2: example description b");
    assert_alt_err!(without_macro(3), "error3: example description c");
}

// -----------------------------------------------------------------------------
// Described errors don't collect error context information.
// -----------------------------------------------------------------------------
#[test]
fn display_with_context_multi() {
    use error_kind::{ErrorContext, error};
    #[error("with_macro failed")]
    fn with_macro(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a")),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b")),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c")),
            _ => Ok(()),
        }
    }
    #[error("with_macro_trailing_comma failed")]
    fn with_macro_trailing_comma(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => with_macro(idx),
            2 => with_macro(idx),
            3 => with_macro(idx),
            _ => Ok(()),
        }
    }
    fn without_macro(idx: u32) -> Result<(), DescribedError> {
        match idx {
            1 => with_macro_trailing_comma(idx).with_context(|| "without_macro failed"),
            2 => with_macro_trailing_comma(idx).with_context(|| "without_macro failed"),
            3 => with_macro_trailing_comma(idx).with_context(|| "without_macro failed"),
            _ => Ok(()),
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // BASIC EQUALITY
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_multi_eq!(
        with_macro(1),
        with_macro_trailing_comma(1),
        without_macro(1),
        Err(DescribedError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(DescribedError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(DescribedError::new(SimpleErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(with_macro(1), "error1: example description a");
    assert_err!(with_macro(2), "error2: example description b");
    assert_err!(with_macro(3), "error3: example description c");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(with_macro(1), "error1: example description a");
    assert_alt_err!(with_macro(2), "error2: example description b");
    assert_alt_err!(with_macro(3), "error3: example description c");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(
        with_macro_trailing_comma(1),
        "error1: example description a"
    );
    assert_err!(
        with_macro_trailing_comma(2),
        "error2: example description b"
    );
    assert_err!(
        with_macro_trailing_comma(3),
        "error3: example description c"
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(
        with_macro_trailing_comma(1),
        "error1: example description a"
    );
    assert_alt_err!(
        with_macro_trailing_comma(2),
        "error2: example description b"
    );
    assert_alt_err!(
        with_macro_trailing_comma(3),
        "error3: example description c"
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(without_macro(1), "error1: example description a");
    assert_err!(without_macro(2), "error2: example description b");
    assert_err!(without_macro(3), "error3: example description c");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_alt_err!(without_macro(1), "error1: example description a");
    assert_alt_err!(without_macro(2), "error2: example description b");
    assert_alt_err!(without_macro(3), "error3: example description c");
}

// -----------------------------------------------------------------------------
#[test]
fn error() {
    use error_kind::Error;
    fn implements_trait<T: core::error::Error + Error<Kind = SimpleErrorKind>>() {}
    implements_trait::<DescribedError>();
    implements_trait::<DescribedErrorAlt>();
    assert_eq!(
        <DescribedError as error_kind::TrivialError>::new(SimpleErrorKind::Error1).error_kind(),
        SimpleErrorKind::Error1,
    );
}
