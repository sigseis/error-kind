# Getting Started

Separate [`Error`] from [`ErrorKind`] with convenient derive macros!

<div class="warning">

**Alpha Software:** This crate is currently in alpha. Until the crate enters beta, the crate has no stability promise and may contain breaking changes with every update.

</div>

## Basic Usage

The most basic usage of `error-kind` is with a simple error/kind combo.

```rust
use error_kind::{Err, Error, ErrorKind};

#[derive(Copy, Clone, Debug, ErrorKind)]
pub enum MyErrorKind {
    SomeFailure,
    SomeOtherFailure,
    // ...
}

#[derive(Clone, Debug, Error)]
pub struct MyError {
    error_kind: MyErrorKind,
}

fn some_function(case: u32) -> Result<(), MyError> {
    match case {
        1 => Err!(MyErrorKind::SomeFailure, "dynamic details: {case}"),
        2 => Err!(MyErrorKind::SomeOtherFailure),
        _ => Ok(()),
    }
}

// This will print:
//   error: some failure
println!("error: {}", some_function(1).unwrap_err());
```

The power of this separation is that you can configure your crate to collect
additional information without changing the surrounding code. This may, however,
affect the size and runtime cost of the containing error.

For example, you can opt-into collection of dynamic error descriptions like this.

```rust
use error_kind::{Err, Error, ErrorKind};

#[derive(Copy, Clone, Debug, ErrorKind)]
pub enum MyErrorKind {
    SomeFailure,
    SomeOtherFailure,
    // ...
}

#[derive(Clone, Debug, Error)]
pub struct MyError {
    envelope: error_kind::envelopes::Described<MyErrorKind>,
}

fn some_function(case: u32) -> Result<(), MyError> {
    match case {
        1 => Err!(MyErrorKind::SomeFailure, "dynamic details: {case}"),
        2 => Err!(MyErrorKind::SomeOtherFailure),
        _ => Ok(()),
    }
}

// Notice we only changed `MyError`?
// This will now print:
//   error: some failure: dynamic details: 1
println!("error: {}", some_function(1).unwrap_err());
```

If you want your users to be able to inspect the error kind, you can grant
access to it by providing a function.

```rust
use error_kind::{Err, Error, ErrorKind};

#[derive(Copy, Clone, Debug, ErrorKind)]
pub enum MyErrorKind {
    SomeFailure,
    SomeOtherFailure,
    // ...
}

#[derive(Clone, Debug, Error)]
pub struct MyError {
    envelope: error_kind::envelopes::Described<MyErrorKind>,
}

impl MyError {
    // Allow the end-user to inspect the error kind!
    pub fn error_kind(&self) -> MyErrorKind {
        use error_kind::envelopes::ErrorEnvelope;
        self.envelope.error_kind()
    }
}
```

## Core Types

This crate is intended to be used by defining types in your own crate, and then implementing the following traits as needed:

| **Trait**       | **Purpose**                                                       |
|-----------------|-------------------------------------------------------------------|
| [`Error`]       | Denotes the type that wraps an [`ErrorKind`].                     |
| [`ErrorKind`]   | Denotes an enum for categorizing semantically-related errors.     |
| [`ErrorDomain`] | Denotes a composition of [`ErrorKind`]s (multiple failure kinds). |

The `Error` type has a supertrait of `core::error::Error`, so it should be compatible with other error handling systems as well.

**All of these error traits support derive macros functionality!**

When defining your error type, you can optionally wrap it in an [`ErrorEnvelope`] to collect additional information.

| **Envelope**   | **Collects**                                                                        |
|----------------|-------------------------------------------------------------------------------------|
| *No Envelope*  | No additional information.                                                          |
| [`Described`]  | The error description stated at the failure-site.                                   |
| [`Diagnostic`] | As much information as possible; backtrace, context, description, source locations. |
| [`Located`]    | The source location where the failure occurred.                                     |

The absence or presence of a wrapping envelope does **not** alter which traits the [`Error`] macro implements.

The only differences are:

- The size of the type that contains the error.
- The runtime cost of the type (e.g. diagnostic envelopes will allocate).
- The information that can be displayed when printing the error.

We try to keep even the traits implemented by an envelope identical, but this may not always be possible.

There are also some additional macros to help make using the system easier.

| **Macro**           | **Kind**   | **Purpose**                                                                                     |
|---------------------|------------|-------------------------------------------------------------------------------------------------|
| [`Err!`] / [`err!`] | Functional | Easily construct an error from an [`ErrorKind`], and accepts optional description information.  |
| [`error`]           | Attribute  | Automatically append error context information when bubbling an error up the stack.             |

The `#[error(...)]` attribute is attached to functions, and automatically appends more error diagnostics to errors propagating up the stack (from the attributed function or deeper).

This attribute can only be used on functions that return `Result`s whose error type implements *error-kind's* [`Error`] trait, however.

```rust
# use std::path::Path;
# #[derive(Clone, Copy, Debug, error_kind::ErrorKind)]
# enum MyErrorKind { Variant }
# #[derive(Clone, Copy, Debug, error_kind::Error)]
# struct MyError(MyErrorKind);
# fn read_file(file: &Path) -> Result<String, MyError> { todo!() }
use error_kind::{error, ResultExt};

// On error, the context "while attempting to read '...'" will be captured.
// This will only be captured if your error uses an appropriate envelope.
#[error("while attempting to read '{file:?}'")]
fn process_file(file: &Path) -> Result<String, MyError> {
    let contents = read_file(file)?; // Some other helper function.
    # todo!("{contents}");
    // ...
}
```

[`Described`]: crate::envelopes::Described
[`Diagnostic`]: crate::envelopes::Diagnostic
[`ErrorEnvelope`]: crate::envelopes::ErrorEnvelope
[`Located`]: crate::envelopes::Located

## When to Use

You should use `error-kind` when you can accept a separation between the [`Error`] and [`ErrorKind`] types.
As demonstrated, this separation is valuable because it grants you flexibility in how you can change your error types as your crate evolves (or based on user configuration).

If you are creating a binary, it's recommended to use an error-erasing crate like [`anyhow`](https://crates.io/crates/anyhow).

If you want error and error kind to be the same type, you should use [`thiserror`](https://crates.io/crates/thiserror).
