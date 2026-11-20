// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use error_kind::ErrorKind;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, ErrorKind, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum ErrorKinds {
    ExampleKind1_2_3Variant,
    EXAMPLEKind4_5_6variant,
    #[error("change the error text")]
    ExampleKind7_8_9Variant,
}

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
#[test]
fn display() {
    assert_eq!(
        format!("{}", ErrorKinds::ExampleKind1_2_3Variant),
        "example kind1 2 3 variant",
    );
    assert_eq!(
        format!("{}", ErrorKinds::EXAMPLEKind4_5_6variant),
        "example kind4 5 6variant",
    );
    assert_eq!(
        format!("{}", ErrorKinds::ExampleKind7_8_9Variant),
        "change the error text",
    );
}

// -----------------------------------------------------------------------------
#[test]
fn error_kind() {
    fn implements_trait<T: ErrorKind>() {}
    implements_trait::<ErrorKinds>();
}
