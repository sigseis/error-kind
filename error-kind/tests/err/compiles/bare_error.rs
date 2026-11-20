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
enum BareErrorKind {
    Error1,
    Error2,
    Error3,
}

// -----------------------------------------------------------------------------
// A bare error kind collects no error information other than the error kind.
// -----------------------------------------------------------------------------
#[derive(Clone, Debug, Error, PartialEq)]
struct BareError {
    error_kind: BareErrorKind,
}

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
#[test]
fn plain() {
    fn with_macro(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err!(BareErrorKind::Error1),
            2 => Err!(BareErrorKind::Error2),
            3 => Err!(BareErrorKind::Error3),
            _ => Ok(()),
        }
    }
    fn with_macro_trailing_comma(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err!(BareErrorKind::Error1,),
            2 => Err!(BareErrorKind::Error2,),
            3 => Err!(BareErrorKind::Error3,),
            _ => Ok(()),
        }
    }
    fn without_macro(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err(BareError::new(BareErrorKind::Error1)),
            2 => Err(BareError::new(BareErrorKind::Error2)),
            3 => Err(BareError::new(BareErrorKind::Error3)),
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
        Err(BareError::new(BareErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(BareError::new(BareErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(BareError::new(BareErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(with_macro(1), "error1");
    assert_both_err!(with_macro(2), "error2");
    assert_both_err!(with_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(with_macro_trailing_comma(1), "error1");
    assert_both_err!(with_macro_trailing_comma(2), "error2");
    assert_both_err!(with_macro_trailing_comma(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(without_macro(1), "error1");
    assert_both_err!(without_macro(2), "error2");
    assert_both_err!(without_macro(3), "error3");
}

// -----------------------------------------------------------------------------
#[test]
fn with_description() {
    fn with_macro(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err!(BareErrorKind::Error1, "example description a"),
            2 => Err!(BareErrorKind::Error2, "example description b"),
            3 => Err!(BareErrorKind::Error3, "example description c"),
            _ => Ok(()),
        }
    }
    fn with_macro_trailing_comma(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err!(BareErrorKind::Error1, "example description a",),
            2 => Err!(BareErrorKind::Error2, "example description b",),
            3 => Err!(BareErrorKind::Error3, "example description c",),
            _ => Ok(()),
        }
    }
    fn without_macro(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err(BareError::with_description(
                BareErrorKind::Error1,
                "example description a",
            )),
            2 => Err(BareError::with_description(
                BareErrorKind::Error2,
                "example description b",
            )),
            3 => Err(BareError::with_description(
                BareErrorKind::Error3,
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
        Err(BareError::new(BareErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_trailing_comma(2),
        without_macro(2),
        Err(BareError::new(BareErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_trailing_comma(3),
        without_macro(3),
        Err(BareError::new(BareErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(with_macro(1), "error1");
    assert_both_err!(with_macro(2), "error2");
    assert_both_err!(with_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(with_macro_trailing_comma(1), "error1");
    assert_both_err!(with_macro_trailing_comma(2), "error2");
    assert_both_err!(with_macro_trailing_comma(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(without_macro(1), "error1");
    assert_both_err!(without_macro(2), "error2");
    assert_both_err!(without_macro(3), "error3");
}

// -----------------------------------------------------------------------------
#[test]
fn with_formatted_description() {
    fn with_macro(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err!(BareErrorKind::Error1, "example description a: idx={idx}"),
            2 => Err!(BareErrorKind::Error2, "example description b: idx={idx}"),
            3 => Err!(BareErrorKind::Error3, "example description c: idx={idx}"),
            _ => Ok(()),
        }
    }
    fn with_macro_extra_args(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err!(BareErrorKind::Error1, "example description a: idx={}", idx),
            2 => Err!(BareErrorKind::Error2, "example description b: idx={}", idx),
            3 => Err!(BareErrorKind::Error3, "example description c: idx={}", idx),
            _ => Ok(()),
        }
    }
    fn without_macro(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err(BareError::with_description(
                BareErrorKind::Error1,
                format!("example description a: idx={idx}"),
            )),
            2 => Err(BareError::with_description(
                BareErrorKind::Error2,
                format!("example description b: idx={idx}"),
            )),
            3 => Err(BareError::with_description(
                BareErrorKind::Error3,
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
        Err(BareError::new(BareErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_extra_args(2),
        without_macro(2),
        Err(BareError::new(BareErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_extra_args(3),
        without_macro(3),
        Err(BareError::new(BareErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(with_macro(1), "error1");
    assert_both_err!(with_macro(2), "error2");
    assert_both_err!(with_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(with_macro_extra_args(1), "error1");
    assert_both_err!(with_macro_extra_args(2), "error2");
    assert_both_err!(with_macro_extra_args(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(without_macro(1), "error1");
    assert_both_err!(without_macro(2), "error2");
    assert_both_err!(without_macro(3), "error3");
}

// -----------------------------------------------------------------------------
#[test]
fn with_error_context() {
    #[error("additional error context")]
    fn with_macro(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err!(BareErrorKind::Error1, "example description a: idx={idx}"),
            2 => Err!(BareErrorKind::Error2, "example description b: idx={idx}"),
            3 => Err!(BareErrorKind::Error3, "example description c: idx={idx}"),
            _ => Ok(()),
        }
    }
    #[error("additional error context")]
    fn with_macro_extra_args(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err!(BareErrorKind::Error1, "example description a: idx={}", idx),
            2 => Err!(BareErrorKind::Error2, "example description b: idx={}", idx),
            3 => Err!(BareErrorKind::Error3, "example description c: idx={}", idx),
            _ => Ok(()),
        }
    }
    fn without_macro(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err(BareError::with_description(
                BareErrorKind::Error1,
                format!("example description a: idx={idx}"),
            )
            .with_context(|| "additional error context")),
            2 => Err(BareError::with_description(
                BareErrorKind::Error2,
                format!("example description b: idx={idx}"),
            )
            .with_context(|| "additional error context")),
            3 => Err(BareError::with_description(
                BareErrorKind::Error3,
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
        Err(BareError::new(BareErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_extra_args(2),
        without_macro(2),
        Err(BareError::new(BareErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_extra_args(3),
        without_macro(3),
        Err(BareError::new(BareErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(with_macro(1), "error1");
    assert_both_err!(with_macro(2), "error2");
    assert_both_err!(with_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(with_macro_extra_args(1), "error1");
    assert_both_err!(with_macro_extra_args(2), "error2");
    assert_both_err!(with_macro_extra_args(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(without_macro(1), "error1");
    assert_both_err!(without_macro(2), "error2");
    assert_both_err!(without_macro(3), "error3");
}

// -----------------------------------------------------------------------------
#[test]
fn with_formatted_error_context() {
    #[error("additional error context: idx={idx}")]
    fn with_macro(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err!(BareErrorKind::Error1, "example description a: idx={idx}"),
            2 => Err!(BareErrorKind::Error2, "example description b: idx={idx}"),
            3 => Err!(BareErrorKind::Error3, "example description c: idx={idx}"),
            _ => Ok(()),
        }
    }
    #[error("additional error context: idx={}", idx)]
    fn with_macro_extra_args(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err!(BareErrorKind::Error1, "example description a: idx={}", idx),
            2 => Err!(BareErrorKind::Error2, "example description b: idx={}", idx),
            3 => Err!(BareErrorKind::Error3, "example description c: idx={}", idx),
            _ => Ok(()),
        }
    }
    fn without_macro(idx: u32) -> Result<(), BareError> {
        match idx {
            1 => Err(BareError::with_description(
                BareErrorKind::Error1,
                format!("example description a: idx={idx}"),
            )
            .with_context(|| format!("additional error context: idx={idx}"))),
            2 => Err(BareError::with_description(
                BareErrorKind::Error2,
                format!("example description b: idx={idx}"),
            )
            .with_context(|| format!("additional error context: idx={idx}"))),
            3 => Err(BareError::with_description(
                BareErrorKind::Error3,
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
        Err(BareError::new(BareErrorKind::Error1)),
    );
    assert_multi_eq!(
        with_macro(2),
        with_macro_extra_args(2),
        without_macro(2),
        Err(BareError::new(BareErrorKind::Error2)),
    );
    assert_multi_eq!(
        with_macro(3),
        with_macro_extra_args(3),
        without_macro(3),
        Err(BareError::new(BareErrorKind::Error3)),
    );

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(with_macro(1), "error1");
    assert_both_err!(with_macro(2), "error2");
    assert_both_err!(with_macro(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITH_MACRO_TRAILING_COMMA
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(with_macro_extra_args(1), "error1");
    assert_both_err!(with_macro_extra_args(2), "error2");
    assert_both_err!(with_macro_extra_args(3), "error3");

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // WITHOUT_MACRO
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    assert_both_err!(without_macro(1), "error1");
    assert_both_err!(without_macro(2), "error2");
    assert_both_err!(without_macro(3), "error3");
}
