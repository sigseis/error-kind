// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use error_kind::{Err, Error, ErrorContext, ErrorKind, TrivialError, error};

// =============================================================================
// HELPER TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, ErrorKind, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum DiagnosticErrorKind {
    Error1,
    Error2,
    Error3,
}

// -----------------------------------------------------------------------------
// A located error will collect the error location information.
// -----------------------------------------------------------------------------
#[derive(Clone, Debug, Error, PartialEq)]
struct DiagnosticError {
    error_kind: error_kind::envelopes::Diagnostic<DiagnosticErrorKind>,
}

// =============================================================================
// TESTS
// -----------------------------------------------------------------------------
// So that these tests are nice and readable, we use a lot of #[rustfmt::skip].
//
// We want to present something as close to what the end user will see as
// possible. This is both so that we can see how it looks, but also so that a
// user browsing the code can see what to expect.
// =============================================================================

// -----------------------------------------------------------------------------
#[test]
fn plain() {
    const WITH_MACRO_OFFSET: u32 = 1;
    const WITH_MACRO_LINE: u32 = line!() + 3;
    fn with_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err!(DiagnosticErrorKind::Error1),
            2 => Err!(DiagnosticErrorKind::Error2),
            3 => Err!(DiagnosticErrorKind::Error3),
            _ => Ok(()),
        }
    }
    const WITH_MACRO_TRAILING_COMMA_OFFSET: u32 = 1;
    const WITH_MACRO_TRAILING_COMMA_LINE: u32 = line!() + 3;
    fn with_macro_trailing_comma(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err!(DiagnosticErrorKind::Error1,),
            2 => Err!(DiagnosticErrorKind::Error2,),
            3 => Err!(DiagnosticErrorKind::Error3,),
            _ => Ok(()),
        }
    }
    const WITHOUT_MACRO_OFFSET: u32 = 1;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(DiagnosticError::new(DiagnosticErrorKind::Error1)),
            2 => Err(DiagnosticError::new(DiagnosticErrorKind::Error2)),
            3 => Err(DiagnosticError::new(DiagnosticErrorKind::Error3)),
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
        Err(DiagnosticError::new(DiagnosticErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1\n  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 0
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2\n  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 1
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3\n  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 2
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1\n  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 0
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2\n  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 1
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3\n  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 2
    );
}

// -----------------------------------------------------------------------------
#[test]
fn with_description() {
    const WITH_MACRO_OFFSET: u32 = 1;
    const WITH_MACRO_LINE: u32 = line!() + 3;
    fn with_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err!(DiagnosticErrorKind::Error1, "example description a"),
            2 => Err!(DiagnosticErrorKind::Error2, "example description b"),
            3 => Err!(DiagnosticErrorKind::Error3, "example description c"),
            _ => Ok(()),
        }
    }
    const WITH_MACRO_TRAILING_COMMA_OFFSET: u32 = 1;
    const WITH_MACRO_TRAILING_COMMA_LINE: u32 = line!() + 3;
    fn with_macro_trailing_comma(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err!(DiagnosticErrorKind::Error1, "example description a",),
            2 => Err!(DiagnosticErrorKind::Error2, "example description b",),
            3 => Err!(DiagnosticErrorKind::Error3, "example description c",),
            _ => Ok(()),
        }
    }
    const WITHOUT_MACRO_OFFSET: u32 = 4;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error1,
                "example description a",
            )),
            2 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error2,
                "example description b",
            )),
            3 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error3,
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
        Err(DiagnosticError::new(DiagnosticErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 0
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 1
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_trailing_comma(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_TRAILING_COMMA_LINE + WITH_MACRO_TRAILING_COMMA_OFFSET * 2
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1: example description a\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 0
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2: example description b\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 1
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3: example description c\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 2
    );
}

// -----------------------------------------------------------------------------
#[test]
fn with_formatted_description() {
    const WITH_MACRO_OFFSET: u32 = 4;
    const WITH_MACRO_LINE: u32 = line!() + 3;
    fn with_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err!(
                DiagnosticErrorKind::Error1,
                "example description a: idx={idx}"
            ),
            2 => Err!(
                DiagnosticErrorKind::Error2,
                "example description b: idx={idx}"
            ),
            3 => Err!(
                DiagnosticErrorKind::Error3,
                "example description c: idx={idx}"
            ),
            _ => Ok(()),
        }
    }
    const WITH_MACRO_EXTRA_ARGS_OFFSET: u32 = 5;
    const WITH_MACRO_EXTRA_ARGS_LINE: u32 = line!() + 3;
    fn with_macro_extra_args(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err!(
                DiagnosticErrorKind::Error1,
                "example description a: idx={}",
                idx
            ),
            2 => Err!(
                DiagnosticErrorKind::Error2,
                "example description b: idx={}",
                idx
            ),
            3 => Err!(
                DiagnosticErrorKind::Error3,
                "example description c: idx={}",
                idx
            ),
            _ => Ok(()),
        }
    }
    const WITHOUT_MACRO_OFFSET: u32 = 4;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error1,
                format!("example description a: idx={idx}"),
            )),
            2 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error2,
                format!("example description b: idx={idx}"),
            )),
            3 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error3,
                format!("example description c: idx={idx}"),
            )),
            _ => Ok(()),
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // BASIC EQUALITY
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_multi_eq!(
        with_macro(1),
        with_macro_extra_args(1),
        without_macro(1),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_extra_args(2),
        without_macro(2),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_extra_args(3),
        without_macro(3),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1: example description a: idx=1\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2: example description b: idx=2\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3: example description c: idx=3\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_EXTRA_ARGS
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_extra_args(1),
        concat!(
            "error1: example description a: idx=1\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_EXTRA_ARGS_LINE + WITH_MACRO_EXTRA_ARGS_OFFSET * 0
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_extra_args(2),
        concat!(
            "error2: example description b: idx=2\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_EXTRA_ARGS_LINE + WITH_MACRO_EXTRA_ARGS_OFFSET * 1
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_extra_args(3),
        concat!(
            "error3: example description c: idx=3\n",
            "  at {file}:{line}:18",
        ),
        file = file!(),
        line = WITH_MACRO_EXTRA_ARGS_LINE + WITH_MACRO_EXTRA_ARGS_OFFSET * 2
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1: example description a: idx=1\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 0
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2: example description b: idx=2\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 1
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3: example description c: idx=3\n",
            "  at {file}:{line}:22",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 2
    );
}

// -----------------------------------------------------------------------------
#[test]
fn with_error_context() {
    const WITH_MACRO_OFFSET: u32 = 4;
    const WITH_MACRO_LINE: u32 = line!() + 4;
    #[error("additional error context")]
    fn with_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err!(
                DiagnosticErrorKind::Error1,
                "example description a: idx={idx}"
            ),
            2 => Err!(
                DiagnosticErrorKind::Error2,
                "example description b: idx={idx}"
            ),
            3 => Err!(
                DiagnosticErrorKind::Error3,
                "example description c: idx={idx}"
            ),
            _ => Ok(()),
        }
    }
    const WITH_MACRO_EXTRA_ARGS_OFFSET: u32 = 5;
    const WITH_MACRO_EXTRA_ARGS_LINE: u32 = line!() + 4;
    #[error("additional error context")]
    fn with_macro_extra_args(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err!(
                DiagnosticErrorKind::Error1,
                "example description a: idx={}",
                idx
            ),
            2 => Err!(
                DiagnosticErrorKind::Error2,
                "example description b: idx={}",
                idx
            ),
            3 => Err!(
                DiagnosticErrorKind::Error3,
                "example description c: idx={}",
                idx
            ),
            _ => Ok(()),
        }
    }
    const WITHOUT_MACRO_OFFSET: u32 = 5;
    const WITHOUT_MACRO_CONTEXT_OFFSET: u32 = 4;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error1,
                format!("example description a: idx={idx}"),
            )
            .with_context(|| "additional error context")),
            2 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error2,
                format!("example description b: idx={idx}"),
            )
            .with_context(|| "additional error context")),
            3 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error3,
                format!("example description c: idx={idx}"),
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
        with_macro_extra_args(1),
        without_macro(1),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_extra_args(2),
        without_macro(2),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_extra_args(3),
        without_macro(3),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1: example description a: idx=1\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
        context = WITH_MACRO_LINE - 3,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2: example description b: idx=2\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
        context = WITH_MACRO_LINE - 3,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3: example description c: idx=3\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
        context = WITH_MACRO_LINE - 3,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_EXTRA_ARGS
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_extra_args(1),
        concat!(
            "error1: example description a: idx=1\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_EXTRA_ARGS_LINE + WITH_MACRO_EXTRA_ARGS_OFFSET * 0,
        context = WITH_MACRO_EXTRA_ARGS_LINE - 3,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_extra_args(2),
        concat!(
            "error2: example description b: idx=2\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_EXTRA_ARGS_LINE + WITH_MACRO_EXTRA_ARGS_OFFSET * 1,
        context = WITH_MACRO_EXTRA_ARGS_LINE - 3,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_extra_args(3),
        concat!(
            "error3: example description c: idx=3\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_EXTRA_ARGS_LINE + WITH_MACRO_EXTRA_ARGS_OFFSET * 2,
        context = WITH_MACRO_EXTRA_ARGS_LINE - 3,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1: example description a: idx=1\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context}:14",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 0,
        context = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2: example description b: idx=2\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context}:14",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 1,
        context = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3: example description c: idx=3\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context\n",
            "       at {file}:{context}:14",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 2,
        context = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 2,
    );
}

// -----------------------------------------------------------------------------
#[test]
fn with_formatted_error_context() {
    const WITH_MACRO_OFFSET: u32 = 4;
    const WITH_MACRO_LINE: u32 = line!() + 4;
    #[error("additional error context: idx={idx}")]
    fn with_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err!(
                DiagnosticErrorKind::Error1,
                "example description a: idx={idx}"
            ),
            2 => Err!(
                DiagnosticErrorKind::Error2,
                "example description b: idx={idx}"
            ),
            3 => Err!(
                DiagnosticErrorKind::Error3,
                "example description c: idx={idx}"
            ),
            _ => Ok(()),
        }
    }
    const WITH_MACRO_EXTRA_ARGS_OFFSET: u32 = 5;
    const WITH_MACRO_EXTRA_ARGS_LINE: u32 = line!() + 4;
    #[error("additional error context: idx={}", idx)]
    fn with_macro_extra_args(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err!(
                DiagnosticErrorKind::Error1,
                "example description a: idx={}",
                idx
            ),
            2 => Err!(
                DiagnosticErrorKind::Error2,
                "example description b: idx={}",
                idx
            ),
            3 => Err!(
                DiagnosticErrorKind::Error3,
                "example description c: idx={}",
                idx
            ),
            _ => Ok(()),
        }
    }
    const WITHOUT_MACRO_OFFSET: u32 = 5;
    const WITHOUT_MACRO_CONTEXT_OFFSET: u32 = 4;
    const WITHOUT_MACRO_LINE: u32 = line!() + 3;
    fn without_macro(idx: u32) -> Result<(), DiagnosticError> {
        match idx {
            1 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error1,
                format!("example description a: idx={idx}"),
            )
            .with_context(|| format!("additional error context: idx={idx}"))),
            2 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error2,
                format!("example description b: idx={idx}"),
            )
            .with_context(|| format!("additional error context: idx={idx}"))),
            3 => Err(DiagnosticError::with_description(
                DiagnosticErrorKind::Error3,
                format!("example description c: idx={idx}"),
            )
            .with_context(|| format!("additional error context: idx={idx}"))),
            _ => Ok(()),
        }
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // BASIC EQUALITY
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_multi_eq!(
        with_macro(1),
        with_macro_extra_args(1),
        without_macro(1),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_extra_args(2),
        without_macro(2),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_extra_args(3),
        without_macro(3),
        Err(DiagnosticError::new(DiagnosticErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(1),
        concat!(
            "error1: example description a: idx=1\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context: idx=1\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 0,
        context = WITH_MACRO_LINE - 3,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(2),
        concat!(
            "error2: example description b: idx=2\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context: idx=2\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 1,
        context = WITH_MACRO_LINE - 3,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro(3),
        concat!(
            "error3: example description c: idx=3\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context: idx=3\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_LINE + WITH_MACRO_OFFSET * 2,
        context = WITH_MACRO_LINE - 3,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_EXTRA_ARGS
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_extra_args(1),
        concat!(
            "error1: example description a: idx=1\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context: idx=1\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_EXTRA_ARGS_LINE + WITH_MACRO_EXTRA_ARGS_OFFSET * 0,
        context = WITH_MACRO_EXTRA_ARGS_LINE - 3,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_extra_args(2),
        concat!(
            "error2: example description b: idx=2\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context: idx=2\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_EXTRA_ARGS_LINE + WITH_MACRO_EXTRA_ARGS_OFFSET * 1,
        context = WITH_MACRO_EXTRA_ARGS_LINE - 3,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        with_macro_extra_args(3),
        concat!(
            "error3: example description c: idx=3\n",
            "  at {file}:{line}:18\n",
            "  with context\n",
            "  ╰─ additional error context: idx=3\n",
            "       at {file}:{context}:5",
        ),
        file = file!(),
        line = WITH_MACRO_EXTRA_ARGS_LINE + WITH_MACRO_EXTRA_ARGS_OFFSET * 2,
        context = WITH_MACRO_EXTRA_ARGS_LINE - 3,
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(1),
        concat!(
            "error1: example description a: idx=1\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context: idx=1\n",
            "       at {file}:{context}:14",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 0,
        context = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 0,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(2),
        concat!(
            "error2: example description b: idx=2\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context: idx=2\n",
            "       at {file}:{context}:14",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 1,
        context = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 1,
    );
    #[rustfmt::skip]
    assert_alt_err!(
        without_macro(3),
        concat!(
            "error3: example description c: idx=3\n",
            "  at {file}:{line}:22\n",
            "  with context\n",
            "  ╰─ additional error context: idx=3\n",
            "       at {file}:{context}:14",
        ),
        file = file!(),
        line = WITHOUT_MACRO_LINE + WITHOUT_MACRO_OFFSET * 2,
        context = WITHOUT_MACRO_LINE + WITHOUT_MACRO_CONTEXT_OFFSET + WITHOUT_MACRO_OFFSET * 2,
    );
}
