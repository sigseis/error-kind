// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
#[test]
fn domain_conversion() {
    // To make the compiler not emit an error about these.
    // (We should test them, but for now let's just get this published.)
    let _ = Error::with_description(SimpleErrorKind::Error2, String::new());

    // Types can return standard error kinds...
    fn alternate_failure() -> Result<(), Error<AlternateErrorKind>> {
        Err(Error::new(AlternateErrorKind::Error1))
    }
    fn simple_failure() -> Result<(), Error<SimpleErrorKind>> {
        Err(Error::new(SimpleErrorKind::Error2))
    }

    // Or they can return error domains...
    fn alternate_domain() -> Result<(), Error<AlternateErrorDomain>> {
        Ok(alternate_failure()?)
    }
    fn simple_domain() -> Result<(), Error<SimpleErrorDomain>> {
        Ok(simple_failure()?)
    }

    // As long as you register your domains, it allows for try-conversion.
    fn complex_domain(idx: u32) -> Result<(), Error<ComplexErrorDomain>> {
        match idx {
            1 => Ok(alternate_failure()?),
            2 => Ok(alternate_domain()?),
            3 => Ok(simple_failure()?),
            4 => Ok(simple_domain()?),
            _ => Ok(()),
        }
    }

    assert_eq!(
        alternate_failure().unwrap_err().error_kind(),
        AlternateErrorKind::Error1,
    );
    assert_eq!(
        simple_failure().unwrap_err().error_kind(),
        SimpleErrorKind::Error2,
    );
    assert_eq!(
        alternate_domain().unwrap_err().error_kind(),
        AlternateErrorDomain::Alternate(AlternateErrorKind::Error1),
    );
    assert_eq!(
        simple_domain().unwrap_err().error_kind(),
        SimpleErrorDomain::Simple(SimpleErrorKind::Error2),
    );
    assert_eq!(
        complex_domain(1).unwrap_err().error_kind(),
        ComplexErrorDomain::Alternate(AlternateErrorKind::Error1),
    );
    assert_eq!(
        complex_domain(2).unwrap_err().error_kind(),
        ComplexErrorDomain::Alternate(AlternateErrorKind::Error1),
    );
    assert_eq!(
        complex_domain(3).unwrap_err().error_kind(),
        ComplexErrorDomain::Simple(SimpleErrorKind::Error2),
    );
    assert_eq!(
        complex_domain(4).unwrap_err().error_kind(),
        ComplexErrorDomain::Simple(SimpleErrorKind::Error2),
    );
}
