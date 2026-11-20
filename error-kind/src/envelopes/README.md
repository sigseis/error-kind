Envelopes an [`ErrorKind`] to collect optional diagnostic information.

- [`Described`] collects an error description.
- [`Diagnostic`] collects all possible diagnostics information.
- [`Located`] collects the location the error occurred.

These types **do not compose**, so you cannot `Described<Located<K>>`.

Using an envelope is optional, `ErrorKind` by itself just ignores all diagnostic information.

[`ErrorKind`]: crate::ErrorKind

## About Envelopes

The [`ErrorEnvelope`] types exist to wrap an [`ErrorKind`], allowing it to collect additional information littered around the codebase during error propagation.

**This allows us to control how detailed our error message will be without changing other parts of the code!**

```rust
use error_kind::{error, Err, Error, ErrorKind};

#[derive(Copy, Clone, Debug, ErrorKind, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MyErrorKind {
    SomeSimpleError,
}

#[derive(Clone, Debug, Error)]
pub struct MyError {
    error: error_kind::envelopes::Diagnostic<MyErrorKind>,
}

#[error("calling some_function with fail={fail}")]
fn some_function(fail: bool) -> Result<(), MyError> {
    if fail {
        return Err!(
            MyErrorKind::SomeSimpleError,
            "asked to fail explicitly",
        );
    }
    Ok(())
}

fn main() {
    let err = some_function(true).unwrap_err();
    println!("error: {err}");
}
```

Since the envelope is [`Diagnostic`], all the additional context is captured and presented!

```text
error: some simple error: asked to fail explicitly
  with context
  ╰─ calling some_function with fail=true
```

The [`Diagnostic`] envelope even collects source lines and backtraces, which can be printed when formatting with the alternate error format (`"{error:#}"`).

**This system takes advantage of the heavy optimizations of the Rust compiler.**

If an envelope type *doesn't* capture some piece of debugging information, it will aggressively inline to suggest to the calling code that the data is not needed.

## Printing Locations and Backtraces

The default display format for any envelopes provided by this crate work in the following way:

- `"{error}"` will print all collected *user-relevant* information.
  - **Will Print:** description, context, and error kind.
  - **Will Omit:** backtraces and source locations.
- `"{error:#}"` will print all collected information.
  - **Will Print:** Everything.
  - **Will Omit:** Nothing.

```rust
# fn some_function(_: bool) -> Result<(), &'static str> { Err("") }
fn main() {
    let err = some_function(true).unwrap_err();

    // Backtrace & locations printed (if captured)!
    println!("error: {err:#}");
}
```

## Accessing Internal Information

**`ErrorEnvelope`s explicitly don't grant access to the information they wrap.**

You should always construct your code in a way in which the information passed into the error envelope is not *necessary*. It's only there to help understand why a failure occurred.

As such, there's no way to inspect any of this "diagnostic" information.
