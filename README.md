<div align="center">
    <a href="https://crates.io/crates/error-kind"><img src="https://img.shields.io/crates/v/error-kind.svg" alt="crates.io"></a>
    <a href="https://crates.io/crates/error-kind"><img src="https://img.shields.io/crates/d/error-kind.svg" alt="Downloads"></a>
    <a href="https://docs.rs/error-kind"><img src="https://docs.rs/error-kind/badge.svg" alt="docs.rs"></a>
</div>

# error-kind

Separate `Error` from `ErrorKind` with convenient derive macros!

> **Alpha Software:** The API is unstable and may contain breaking changes in
> any release. Do not depend on `error-kind` for a stable public interface yet.

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

See the [full documentation](https://docs.rs/error-kind) to learn about all the
different error envelopes, as well as some complex use-cases like composing and
organizing related error kinds into `ErrorDomain`s!

## When to Use

You should use `error-kind` when you can accept a separation between the `Error`
and `ErrorKind` types. As demonstrated, this separation is valuable because it
grants you flexibility in how you can change your error types as your crate
evolves (or based on user configuration).

If you are creating a binary, it's recommended to use an error-erasing crate
like [`anyhow`](https://crates.io/crates/anyhow).

If you want error and error kind to be the same type, you could use
[`thiserror`](https://crates.io/crates/thiserror).

## Changelog

See [CHANGELOG](https://github.com/sigseis/error-kind/blob/main/CHANGELOG.md)
for release history.

## Contributing

We welcome contributions!

Please read the
[CONTRIBUTING](https://github.com/sigseis/error-kind/blob/main/CONTRIBUTING.md)
guide before contributing.

## License

Licensed under either the
[Apache License, Version 2.0](https://github.com/sigseis/error-kind/blob/main/LICENSE-APACHE)
or [MIT license](https://github.com/sigseis/error-kind/blob/main/LICENSE-MIT),
at your option.
