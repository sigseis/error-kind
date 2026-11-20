// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::details::StaticString;
use crate::envelopes::alloc::ContextEntry;
use core::fmt::Formatter;
use core::panic::Location;
#[cfg(feature = "backtrace")]
use std::backtrace::{Backtrace, BacktraceStatus};
use std_alloc::string::String;
#[cfg(feature = "backtrace")]
use std_alloc::sync::Arc;
use std_alloc::vec::Vec;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Clone)]
pub(crate) struct ErrorMetadata {
    #[cfg(feature = "backtrace")]
    pub backtrace: Option<Arc<Backtrace>>,
    pub context: Vec<ContextEntry>,
    pub description: String,
    pub location: Option<&'static Location<'static>>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl ErrorMetadata {
    #[inline]
    pub fn new<S>(description: S, location: Option<&'static Location<'static>>) -> Self
    where
        S: StaticString,
    {
        Self {
            #[cfg(feature = "backtrace")]
            backtrace: {
                let backtrace = Backtrace::capture();
                match backtrace.status() {
                    BacktraceStatus::Disabled => None,
                    _ => Some(Arc::new(backtrace)),
                }
            },
            context: Vec::new(),
            description: description.into_owned(),
            location,
        }
    }

    #[track_caller]
    #[inline]
    pub fn write_metadata(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        if !self.description.is_empty() {
            write!(f, ": {}", self.description)?;
        }
        if f.alternate()
            && let Some(location) = self.location
        {
            write!(f, "\n  at {}", location)?;
        }
        if !self.context.is_empty() {
            let mut contexts = self.context.iter().peekable();
            write!(f, "\n  with context")?;
            while let Some(context) = contexts.next() {
                let (start, next) = match contexts.peek() {
                    Some(_) => ("├─", "│"),
                    None => ("╰─", " "),
                };
                write!(f, "\n  {start} {}", context.data)?;
                if f.alternate() {
                    write!(f, "\n  {next}    at {}", context.location)?;
                }
            }
        }
        #[cfg(feature = "backtrace")]
        if f.alternate()
            && let Some(backtrace) = &self.backtrace
        {
            write!(f, "\n  with backtrace")?;
            write!(f, "\n{:#}", backtrace)?;
        }
        Ok(())
    }
}
