Attach context automatically at the end of a failing function.

A common operation in error handling is to want to attach some common context when returning from a failing function. Often, this is handled by attaching the context at the call-site.

However, in some cases, we always want to attach the same kind of context regardless of the call-site. For example, when we're and reading a file, we may wish to attach the filename.

# Examples

```rust
# use std::path::Path;
# #[derive(Clone, Copy, Debug, error_kind::ErrorKind)]
# enum MyErrorKind { Variant }
# #[derive(Clone, Copy, Debug, error_kind::Error)]
# struct MyError(MyErrorKind);
use error_kind::{error, ResultExt};

#[error("while attempting to read '{file:?}'")]
fn read_file(file: &Path) -> Result<String, MyError> {
    // Capture the filesystem error as context as well.
    Result::capture(MyErrorKind::Variant, std::fs::read_to_string(file))
}
```

The above is equivalent to this:

```rust
# use std::path::Path;
# #[derive(Clone, Copy, Debug, error_kind::ErrorKind)]
# enum MyErrorKind { Variant }
# #[derive(Clone, Copy, Debug, error_kind::Error)]
# struct MyError(MyErrorKind);
use error_kind::{error, ResultExt};

fn read_file(file: &Path) -> Result<String, MyError> {
    let body = || {
        Result::capture(MyErrorKind::Variant, std::fs::read_to_string(file))
    };
    let result: Result<String, MyError> = body();
    result.with_context(|| format!("while attempting to read '{file:?}'"))
}
```
