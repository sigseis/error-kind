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
    implements_trait::<DiagnosticError>();
    implements_trait::<DiagnosticErrorAlt>();
    // The implementation for this is validated in `display` tests.
}

// -----------------------------------------------------------------------------
#[test]
fn display() {
    const WITH_MACRO_OFFSET: u32 = 1;
    const WITH_MACRO_LINE: u32 = line!() + 3;
    fn with_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1)),
            2 => Err(err!(SimpleErrorKind::Error2)),
            3 => Err(err!(SimpleErrorKind::Error3)),
            _ => Ok(()),
        }
    }

    const WITH_MACRO_TRAILING_COMMA_OFFSET: u32 = 1;
    const WITH_MACRO_TRAILING_COMMA_LINE: u32 = line!() + 3;
    fn with_macro_trailing_comma(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1,)),
            2 => Err(err!(SimpleErrorKind::Error2,)),
            3 => Err(err!(SimpleErrorKind::Error3,)),
            _ => Ok(()),
        }
    }

    const WITHOUT_MACRO_OFFSET: u32 = 1;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(DiagnosticError::new(SimpleErrorKind::Error1)),
            2 => Err(DiagnosticError::new(SimpleErrorKind::Error2)),
            3 => Err(DiagnosticError::new(SimpleErrorKind::Error3)),
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
        Err(DiagnosticError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(DiagnosticError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(DiagnosticError::new(SimpleErrorKind::Error3)),
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
#[test]
fn display_with_description() {
    const WITH_MACRO_OFFSET: u32 = 1;
    const WITH_MACRO_LINE: u32 = line!() + 3;
    fn with_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a")),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b")),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c")),
            _ => Ok(()),
        }
    }

    const WITH_MACRO_TRAILING_COMMA_OFFSET: u32 = 1;
    const WITH_MACRO_TRAILING_COMMA_LINE: u32 = line!() + 3;
    fn with_macro_trailing_comma(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a",)),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b",)),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c",)),
            _ => Ok(()),
        }
    }

    const WITHOUT_MACRO_OFFSET: u32 = 4;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(DiagnosticError::with_description(
                SimpleErrorKind::Error1,
                "example description a",
            )),
            2 => Err(DiagnosticError::with_description(
                SimpleErrorKind::Error2,
                "example description b",
            )),
            3 => Err(DiagnosticError::with_description(
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
        Err(DiagnosticError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(DiagnosticError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(DiagnosticError::new(SimpleErrorKind::Error3)),
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
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
    );

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
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 2,
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
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 2,
    );
}

// -----------------------------------------------------------------------------
#[test]
fn display_with_context() {
    use error_kind::{ErrorContext, error};

    const WITH_MACRO_CONTEXT_LINE: u32 = line!() + 3;
    const WITH_MACRO_OFFSET: u32 = 1;
    const WITH_MACRO_LINE: u32 = line!() + 4;
    #[error("additional error context")]
    fn with_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a")),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b")),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c")),
            _ => Ok(()),
        }
    }

    const WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE: u32 = line!() + 3;
    const WITH_MACRO_TRAILING_COMMA_OFFSET: u32 = 1;
    const WITH_MACRO_TRAILING_COMMA_LINE: u32 = line!() + 4;
    #[error("additional error context")]
    fn with_macro_trailing_comma(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a",)),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b",)),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c",)),
            _ => Ok(()),
        }
    }

    const WITHOUT_MACRO_CONTEXT_OFFSET: u32 = 4;
    const WITHOUT_MACRO_OFFSET: u32 = 5;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(DiagnosticError::with_description(
                SimpleErrorKind::Error1,
                "example description a",
            )
            .with_context(|| "additional error context")),
            2 => Err(DiagnosticError::with_description(
                SimpleErrorKind::Error2,
                "example description b",
            )
            .with_context(|| "additional error context")),
            3 => Err(DiagnosticError::with_description(
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
        Err(DiagnosticError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(DiagnosticError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(DiagnosticError::new(SimpleErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_err!(
        with_macro(1),
        concat!(
            "error1: example description a\n",
            "  with context\n",
            "  ╰─ additional error context",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        with_macro(2),
        concat!(
            "error2: example description b\n",
            "  with context\n",
            "  ╰─ additional error context",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        with_macro(3),
        concat!(
            "error3: example description c\n",
            "  with context\n",
            "  ╰─ additional error context",
        )
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
        context_line = WITH_MACRO_CONTEXT_LINE,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
        context_line = WITH_MACRO_CONTEXT_LINE,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
        context_line = WITH_MACRO_CONTEXT_LINE,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1: example description a\n",
            "  with context\n",
            "  ╰─ additional error context",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2: example description b\n",
            "  with context\n",
            "  ╰─ additional error context",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3: example description c\n",
            "  with context\n",
            "  ╰─ additional error context",
        )
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 0,
        context_line = WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 1,
        context_line = WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 2,
        context_line = WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_err!(
        without_macro(1),
        concat!(
            "error1: example description a\n",
            "  with context\n",
            "  ╰─ additional error context",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        without_macro(2),
        concat!(
            "error2: example description b\n",
            "  with context\n",
            "  ╰─ additional error context",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        without_macro(3),
        concat!(
            "error3: example description c\n",
            "  with context\n",
            "  ╰─ additional error context",
        )
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context_line}:14",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 0,
        context_line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context_line}:14",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 1,
        context_line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context_line}:14",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 2,
        context_line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 2,
    );
}

// -----------------------------------------------------------------------------
#[test]
fn display_with_context_multi() {
    use error_kind::{ErrorContext, error};

    const WITH_MACRO_CONTEXT_LINE: u32 = line!() + 3;
    const WITH_MACRO_OFFSET: u32 = 1;
    const WITH_MACRO_LINE: u32 = line!() + 4;
    #[error("with_macro failure")]
    fn with_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(err!(SimpleErrorKind::Error1, "example description a")),
            2 => Err(err!(SimpleErrorKind::Error2, "example description b")),
            3 => Err(err!(SimpleErrorKind::Error3, "example description c")),
            _ => Ok(()),
        }
    }

    // Bubbling up the error only will point at the actual error location.
    const WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE: u32 = line!() + 1;
    #[error("with_macro_trailing_comma failure")]
    fn with_macro_trailing_comma(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => with_macro(idx),
            2 => with_macro(idx),
            3 => with_macro(idx),
            _ => Ok(()),
        }
    }

    // Bubbling up the error only will point at the actual error location.
    const WITHOUT_MACRO_CONTEXT_OFFSET: u32 = 0;
    const WITHOUT_MACRO_OFFSET: u32 = 1;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => with_macro_trailing_comma(idx).with_context(|| "without_macro failure"),
            2 => with_macro_trailing_comma(idx).with_context(|| "without_macro failure"),
            3 => with_macro_trailing_comma(idx).with_context(|| "without_macro failure"),
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
        Err(DiagnosticError::new(SimpleErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(DiagnosticError::new(SimpleErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(DiagnosticError::new(SimpleErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_err!(
        with_macro(1),
        concat!(
            "error1: example description a\n",
            "  with context\n",
            "  ╰─ with_macro failure",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        with_macro(2),
        concat!(
            "error2: example description b\n",
            "  with context\n",
            "  ╰─ with_macro failure",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        with_macro(3),
        concat!(
            "error3: example description c\n",
            "  with context\n",
            "  ╰─ with_macro failure",
        )
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ with_macro failure\n",
            "       at {file}:{with_macro_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
        with_macro_line = WITH_MACRO_CONTEXT_LINE,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ with_macro failure\n",
            "       at {file}:{with_macro_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
        with_macro_line = WITH_MACRO_CONTEXT_LINE,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ with_macro failure\n",
            "       at {file}:{with_macro_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
        with_macro_line = WITH_MACRO_CONTEXT_LINE,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1: example description a\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  ╰─ with_macro_trailing_comma failure",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2: example description b\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  ╰─ with_macro_trailing_comma failure",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3: example description c\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  ╰─ with_macro_trailing_comma failure",
        )
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  │    at {file}:{with_macro_line}:5\n",
            "  ╰─ with_macro_trailing_comma failure\n",
            "       at {file}:{with_macro_trailing_comma_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
        with_macro_line = WITH_MACRO_CONTEXT_LINE,
        with_macro_trailing_comma_line = WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  │    at {file}:{with_macro_line}:5\n",
            "  ╰─ with_macro_trailing_comma failure\n",
            "       at {file}:{with_macro_trailing_comma_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
        with_macro_line = WITH_MACRO_CONTEXT_LINE,
        with_macro_trailing_comma_line = WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  │    at {file}:{with_macro_line}:5\n",
            "  ╰─ with_macro_trailing_comma failure\n",
            "       at {file}:{with_macro_trailing_comma_line}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
        with_macro_line = WITH_MACRO_CONTEXT_LINE,
        with_macro_trailing_comma_line = WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_err!(
        without_macro(1),
        concat!(
            "error1: example description a\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  ├─ with_macro_trailing_comma failure\n",
            "  ╰─ without_macro failure",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        without_macro(2),
        concat!(
            "error2: example description b\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  ├─ with_macro_trailing_comma failure\n",
            "  ╰─ without_macro failure",
        )
    );
    #[rustfmt::skip]
    assert_err!(
        without_macro(3),
        concat!(
            "error3: example description c\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  ├─ with_macro_trailing_comma failure\n",
            "  ╰─ without_macro failure",
        )
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO (ALTERNATE)
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  │    at {file}:{with_macro_line}:5\n",
            "  ├─ with_macro_trailing_comma failure\n",
            "  │    at {file}:{with_macro_trailing_comma_line}:5\n",
            "  ╰─ without_macro failure\n",
            "       at {file}:{without_macro_line}:49",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
        with_macro_line = WITH_MACRO_CONTEXT_LINE,
        with_macro_trailing_comma_line = WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE,
        without_macro_line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  │    at {file}:{with_macro_line}:5\n",
            "  ├─ with_macro_trailing_comma failure\n",
            "  │    at {file}:{with_macro_trailing_comma_line}:5\n",
            "  ╰─ without_macro failure\n",
            "       at {file}:{without_macro_line}:49",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
        with_macro_line = WITH_MACRO_CONTEXT_LINE,
        with_macro_trailing_comma_line = WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE,
        without_macro_line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ├─ with_macro failure\n",
            "  │    at {file}:{with_macro_line}:5\n",
            "  ├─ with_macro_trailing_comma failure\n",
            "  │    at {file}:{with_macro_trailing_comma_line}:5\n",
            "  ╰─ without_macro failure\n",
            "       at {file}:{without_macro_line}:49",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
        with_macro_line = WITH_MACRO_CONTEXT_LINE,
        with_macro_trailing_comma_line = WITH_MACRO_TRAILING_COMMA_CONTEXT_LINE,
        without_macro_line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 2,
    );
}

// -----------------------------------------------------------------------------
#[test]
fn error() {
    use error_kind::Error;
    fn implements_trait<T: core::error::Error + Error<Kind = SimpleErrorKind>>() {}
    implements_trait::<DiagnosticError>();
    implements_trait::<DiagnosticErrorAlt>();
    assert_eq!(
        <DiagnosticError as error_kind::TrivialError>::new(SimpleErrorKind::Error1).error_kind(),
        SimpleErrorKind::Error1,
    );
}
