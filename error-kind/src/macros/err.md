Uses type inference to construct trivial error types with a provided [`ErrorKind`].

Calling this macro is equivalent to calling one of the following trait functions:

- `TrivialError::new($error_kind.into())`
- `TrivialError::with_description($error_kind.into(), format!($args))`

In Rust, this can compile as long as the left-hand side of the expression has an explicit type (the variable we're binding it to, or the result of a function).

In order to use this macro, your error type needs to implement the [`TrivialError`] trait. This is automatically implemented for you if your error type is derived with only a single field (see [`Error`] for details on this).

## no-alloc

If the `alloc` feature is not enabled for this crate, the description information will be checked using `format_args!`, but it will not be used (since we cannot allocate a string).

This ensures you can still write `err!(error_kind, "description: {}", value)`, regardless of how your crate is configured.

This is one of the benefits of using a macro here. You can litter around details that you compile out of a production library.

The idea being that a library will only incur the cost if:

- It's possible to allocate (feature = "alloc")
- Your error collects the description (see [`envelopes`])

[`ErrorKind`]: crate::ErrorKind
[`Error`]: crate::Error
[`TrivialError`]: crate::TrivialError
[`envelopes`]: crate::envelopes

## Examples

This macro is intended to be used for the really common pattern of returning with an error from a function.

```rust
use error_kind::err;
# #[derive(Copy, Clone, Debug, error_kind::ErrorKind)]
# enum MyErrorKind { Variant }
# #[derive(Clone, Debug, error_kind::Error)]
# struct MyError(MyErrorKind);

fn example() -> Result<(), MyError> {
    # let some_check = false;
    // ..
    if !some_check {
        return Err(err!(MyErrorKind::Variant, "description"));
    }
    Ok(())
}
```

For added convenience, an [`Err!`] macro is also exported, which wraps even the `Err(_)` part of this operation.

```rust
use error_kind::Err;
# #[derive(Copy, Clone, Debug, error_kind::ErrorKind)]
# enum MyErrorKind { Variant }
# #[derive(Clone, Debug, error_kind::Error)]
# struct MyError(MyErrorKind);

fn example() -> Result<(), MyError> {
    # let some_check = false;
    // ..
    if !some_check {
        return Err!(MyErrorKind::Variant, "description");
    }
    Ok(())
}
```

If you have an aversion to macros, or you don't want to be able to compile-out formatting based on alloc support, you can just call the constructors directly.

```rust
use error_kind::TrivialError;
# #[derive(Copy, Clone, Debug, error_kind::ErrorKind)]
# enum MyErrorKind { Variant }
# #[derive(Clone, Debug, error_kind::Error)]
# struct MyError(MyErrorKind);

fn example() -> Result<(), MyError> {
    # let some_check = false;
    // ..
    if !some_check {
        return Err(MyError::with_description(
            MyErrorKind::Variant,
            "description",
        ));
    }
    Ok(())
}
```

[`Err!`]: crate::Err
