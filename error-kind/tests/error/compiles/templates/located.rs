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
    implements_trait::<LocatedError>();
    implements_trait::<LocatedErrorAlt>();
    // The implementation for this is validated in `display` tests.
}

// -----------------------------------------------------------------------------
#[test]
fn display() {
    const WITH_MACRO_OFFSET: u32 = 1;
    const WITH_MACRO_LINE: u32 = line!() + 3;
    fn with_macro(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1)),
            2 => Err(err!(SimpleErrorKind::Error2)),
            3 => Err(err!(SimpleErrorKind::Error3)),
            _ => Ok(()),
        }
    }

    const WITH_MACRO_TRAILING_COMMA_OFFSET: u32 = 1;
    const WITH_MACRO_TRAILING_COMMA_LINE: u32 = line!() + 3;
    fn with_macro_trailing_comma(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1,)),
            2 => Err(err!(SimpleErrorKind::Error2,)),
            3 => Err(err!(SimpleErrorKind::Error3,)),
            _ => Ok(()),
        }
    }

    const WITHOUT_MACRO_OFFSET: u32 = 1;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => Err(LocatedError::new(SimpleErrorKind::Error1)),
            2 => Err(LocatedError::new(SimpleErrorKind::Error2)),
            3 => Err(LocatedError::new(SimpleErrorKind::Error3)),
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
        Err(LocatedError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(LocatedError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(LocatedError::new(SimpleErrorKind::Error3)),
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
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(with_macro_trailing_comma(1), "error1");
    assert_err!(with_macro_trailing_comma(2), "error2");
    assert_err!(with_macro_trailing_comma(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 2,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(without_macro(1), "error1");
    assert_err!(without_macro(2), "error2");
    assert_err!(without_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 2,
    );
}

// -----------------------------------------------------------------------------
// Located errors don't collect error description information.
// -----------------------------------------------------------------------------
#[test]
fn display_with_description() {
    const WITH_MACRO_OFFSET: u32 = 1;
    const WITH_MACRO_LINE: u32 = line!() + 3;
    fn with_macro(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a")),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b")),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c")),
            _ => Ok(()),
        }
    }

    const WITH_MACRO_TRAILING_COMMA_OFFSET: u32 = 1;
    const WITH_MACRO_TRAILING_COMMA_LINE: u32 = line!() + 3;
    fn with_macro_trailing_comma(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a",)),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b",)),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c",)),
            _ => Ok(()),
        }
    }

    const WITHOUT_MACRO_OFFSET: u32 = 4;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => Err(LocatedError::with_description(
                SimpleErrorKind::Error1,
                "example description a",
            )),
            2 => Err(LocatedError::with_description(
                SimpleErrorKind::Error2,
                "example description b",
            )),
            3 => Err(LocatedError::with_description(
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
        Err(LocatedError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(LocatedError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(LocatedError::new(SimpleErrorKind::Error3)),
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
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(with_macro_trailing_comma(1), "error1");
    assert_err!(with_macro_trailing_comma(2), "error2");
    assert_err!(with_macro_trailing_comma(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 2,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(without_macro(1), "error1");
    assert_err!(without_macro(2), "error2");
    assert_err!(without_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 2,
    );
}

// -----------------------------------------------------------------------------
// Located errors don't collect error context information.
// -----------------------------------------------------------------------------
#[test]
fn display_with_context() {
    use error_kind::{ErrorContext, error};

    const WITH_MACRO_OFFSET: u32 = 1;
    const WITH_MACRO_LINE: u32 = line!() + 4;
    #[error("additional error context")]
    fn with_macro(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a")),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b")),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c")),
            _ => Ok(()),
        }
    }

    const WITH_MACRO_TRAILING_COMMA_OFFSET: u32 = 1;
    const WITH_MACRO_TRAILING_COMMA_LINE: u32 = line!() + 4;
    #[error("additional error context")]
    fn with_macro_trailing_comma(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a",)),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b",)),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c",)),
            _ => Ok(()),
        }
    }

    const WITHOUT_MACRO_OFFSET: u32 = 5;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => Err(LocatedError::with_description(
                SimpleErrorKind::Error1,
                "example description a",
            )
            .with_context(|| "additional error context")),
            2 => Err(LocatedError::with_description(
                SimpleErrorKind::Error2,
                "example description b",
            )
            .with_context(|| "additional error context")),
            3 => Err(LocatedError::with_description(
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
        Err(LocatedError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(LocatedError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(LocatedError::new(SimpleErrorKind::Error3)),
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
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(with_macro_trailing_comma(1), "error1");
    assert_err!(with_macro_trailing_comma(2), "error2");
    assert_err!(with_macro_trailing_comma(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 2,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(without_macro(1), "error1");
    assert_err!(without_macro(2), "error2");
    assert_err!(without_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 2,
    );
}

// -----------------------------------------------------------------------------
// Located errors don't collect error context information.
// -----------------------------------------------------------------------------
#[test]
fn display_with_context_multi() {
    use error_kind::{ErrorContext, error};

    const WITH_MACRO_OFFSET: u32 = 1;
    const WITH_MACRO_LINE: u32 = line!() + 4;
    #[error("with_macro failed")]
    fn with_macro(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a")),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b")),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c")),
            _ => Ok(()),
        }
    }

    // Bubbling up the error only will point at the actual error location.
    #[error("with_macro_trailing_comma failed")]
    fn with_macro_trailing_comma(idx: u32) -> Result<(), LocatedError> {
        match idx {
            1 => with_macro(idx),
            2 => with_macro(idx),
            3 => with_macro(idx),
            _ => Ok(()),
        }
    }

    // Bubbling up the error only will point at the actual error location.
    fn without_macro(idx: u32) -> Result<(), LocatedError> {
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
        Err(LocatedError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(LocatedError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(LocatedError::new(SimpleErrorKind::Error3)),
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
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(with_macro_trailing_comma(1), "error1");
    assert_err!(with_macro_trailing_comma(2), "error2");
    assert_err!(with_macro_trailing_comma(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_err!(without_macro(1), "error1");
    assert_err!(without_macro(2), "error2");
    assert_err!(without_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
    );
}

// -----------------------------------------------------------------------------
#[test]
fn error() {
    use error_kind::Error;
    fn implements_trait<T: core::error::Error + Error<Kind = SimpleErrorKind>>() {}
    implements_trait::<LocatedError>();
    implements_trait::<LocatedErrorAlt>();
    assert_eq!(
        <LocatedError as error_kind::TrivialError>::new(SimpleErrorKind::Error1).error_kind(),
        SimpleErrorKind::Error1,
    );
}
