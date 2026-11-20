// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use super::*;
#[cfg(feature = "alloc")]
use core::assert_matches;

// =============================================================================
// TESTS
// =============================================================================

// -----------------------------------------------------------------------------
mod r#str {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn into_cow() {
        assert_eq!("example".into_cow(), "example");
        assert_matches!("example".into_cow(), Cow::Borrowed(_));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn into_owned() {
        assert_eq!("example".into_owned(), String::from("example"));
    }

    #[test]
    fn try_borrow() {
        assert_eq!("example".try_borrow(), Some("example"));
    }
}

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
mod cow {
    use super::*;

    #[test]
    fn into_cow() {
        assert_eq!(Cow::Borrowed("example").into_cow(), "example",);
        assert_eq!(
            Cow::<str>::Owned(String::from("example")).into_cow(),
            "example",
        );
        assert_matches!(Cow::Borrowed("example").into_cow(), Cow::Borrowed(_),);
        assert_matches!(
            Cow::<str>::Owned(String::from("example")).into_cow(),
            Cow::Owned(_),
        );
    }

    #[test]
    fn into_owned() {
        assert_eq!(
            Cow::Borrowed("example").into_owned(),
            String::from("example"),
        );
        assert_eq!(
            Cow::<str>::Owned(String::from("example")).into_owned(),
            String::from("example"),
        );
    }

    #[test]
    fn try_borrow() {
        assert_eq!(Cow::Borrowed("example").try_borrow(), Some("example"),);
        assert_eq!(
            Cow::<str>::Owned(String::from("example")).try_borrow(),
            None,
        );
    }
}

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
mod string {
    use super::*;

    #[test]
    fn into_cow() {
        assert_eq!(String::from("example").into_cow(), String::from("example"),);
        assert_matches!(String::from("example").into_cow(), Cow::Owned(_),);
    }

    #[test]
    fn into_owned() {
        assert_eq!(
            String::from("example").into_owned(),
            String::from("example"),
        );
    }

    #[test]
    fn try_borrow() {
        assert_eq!(String::from("example").try_borrow(), None,);
    }
}
