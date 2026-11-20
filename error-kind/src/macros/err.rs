// =============================================================================
// MACROS
// =============================================================================

// -----------------------------------------------------------------------------
cfg_select! {
    feature = "alloc" => {
        #[macro_export]
        #[doc = include_str!("err.md")]
        macro_rules! err {
            ($error_kind:expr $(,)?) => {
                $crate::TrivialError::new($error_kind.into())
            };
            ($error_kind:expr, $($arg:tt)*) => {
                $crate::TrivialError::with_description(
                    $error_kind.into(),
                    $crate::__private::format!($($arg)*),
                )
            };
        }
    },
    _ => {
        #[macro_export]
        #[doc = include_str!("err.md")]
        macro_rules! err {
            ($error_kind:expr $(,)?) => {
                $crate::TrivialError::new($error_kind.into())
            };
            ($error_kind:expr, $($arg:tt)*) => {{
                let _ = $crate::__private::format_args!($($arg)*);
                $crate::TrivialError::new($error_kind.into())
            }};
        }
    },
}

/// Wraps a provided error kind and optional description in an error.
///
/// This is equivalent to typing `Err(err!(...))`. See the [`err!`] macro for
/// more information.
///
/// > **NOTE:** The `Err!` macro does not conflict with the `Err` result
/// > variant. You can mix your usage of these two items if you want to.
/// >
/// > If you don't like using this macro, that's totally fine as well. :)
#[macro_export]
macro_rules! Err {
    ($($tt:tt)*) => { Err($crate::err!($($tt)*)) };
}
