// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(test)]
#[path = "diagnostic.tests.rs"]
mod tests;

// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::details::StaticString;
use crate::envelopes::alloc::{ContextEntry, ErrorMetadata};
use crate::envelopes::{Described, ErrorEnvelope, Located};
use crate::{Context, ErrorContext, ErrorKind};
use core::fmt::{Debug, Display, Formatter};
use core::panic::Location;
use std_alloc::string::String;
use std_alloc::sync::Arc;

// =============================================================================
// TYPES
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Clone)]
struct BoxedData<K> {
    pub kind: K,
    pub metadata: ErrorMetadata,
}

/// An envelope that collects detailed diagnostic information.
///
/// The backtrace will only be collected if the `backtrace` feature is enabled,
/// *and* either `RUST_BACKTRACE` or `RUST_LIB_BACKTRACE` are set.
///
/// # Collects
///
/// - backtrace *(if enabled)*
/// - context
/// - description
/// - error kind
/// - locations
///
/// # When To Use
///
/// When you are debugging an issue with your crate, or you want to allow a
/// dependant to debug an issue with your crate.
///
/// # Example Display
///
/// When printed normally (`"{error}"`):
///
/// ```text
/// static error code: detailed dynamic error message
///   with context
///   ├─ we found that a (12) is not b (34)
///   ╰─ while working in my_procedure (a: 12, b: 34)
/// ```
///
/// When printed in alternate form (`"{error:#}"`):
///
/// ```text
/// static error code: detailed dynamic error message
///   at my-crate/src/main.rs:25:16
///   with context
///   ├─ we found that a (12) is not b (34)
///   │    at my-crate/src/main.rs:25:16
///   ╰─ while working in my_procedure (a: 12, b: 34)
///        at my-crate/src/main.rs:22:1
/// ```
#[derive(Clone)]
pub struct Diagnostic<K> {
    // TODO: Perf Test: It might be better to move `kind` out here?
    // TODO: Does this *need* to be Send/Sync ala `Arc`; why not `Box`?
    //       One reason it might need to be is to keep the traits implemented
    //       as similar as possible between error envelopes - let's think about
    //       this more, however. (The whole `Diagnostic` envelope, really.)
    inner: Arc<BoxedData<K>>,
}

// =============================================================================
// IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
impl<K: ErrorKind> Diagnostic<K> {
    #[inline]
    pub(crate) fn into_description(self) -> String {
        self.into_metadata().description
    }

    #[inline]
    pub(crate) fn into_location(self) -> Option<&'static Location<'static>> {
        self.into_metadata().location
    }

    #[inline]
    pub(crate) fn into_metadata(self) -> ErrorMetadata {
        Arc::unwrap_or_clone(self.inner).metadata
    }
}

// -----------------------------------------------------------------------------
impl<K> Diagnostic<K> {
    #[inline]
    pub(crate) fn error_kind_ref(&self) -> &K {
        &self.inner.kind
    }
}

// =============================================================================
// TRAIT IMPLS
// =============================================================================

// -----------------------------------------------------------------------------
crate::envelopes::impl_error_adapter_traits! {
    others={
        alloc=[Described],
        core=[Located],
    },
    type=Diagnostic,
}

// -----------------------------------------------------------------------------
impl<K: Debug> Debug for Diagnostic<K> {
    #[track_caller]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let inner = &*self.inner;
        cfg_select! {
            feature = "backtrace" => f.debug_struct("Diagnostic")
            .field("backtrace", &inner.metadata.backtrace)
            .field("context", &inner.metadata.context)
            .field("description", &inner.metadata.description)
            .field("kind", &inner.kind)
            .field("location", &inner.metadata.location)
            .finish(),
           _ => f.debug_struct("Diagnostic")
            .field("context", &inner.metadata.context)
            .field("description", &inner.metadata.description)
            .field("kind", &inner.kind)
            .field("location", &inner.metadata.location)
            .finish(),
        }
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> Display for Diagnostic<K> {
    #[track_caller]
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let inner = &*self.inner;
        Display::fmt(&inner.kind, f)?;
        inner.metadata.write_metadata(f)
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> ErrorEnvelope for Diagnostic<K> {
    type Kind = K;
    type Envelope<T: ErrorKind> = Diagnostic<T>;

    #[track_caller]
    #[inline]
    fn convert<T>(self) -> Self::Envelope<T>
    where
        T: ErrorKind + From<Self::Kind>,
    {
        let inner = Arc::unwrap_or_clone(self.inner);
        Diagnostic {
            inner: Arc::new(BoxedData {
                kind: From::from(inner.kind),
                metadata: inner.metadata,
            }),
        }
    }

    #[track_caller]
    #[inline]
    fn error_kind(&self) -> Self::Kind {
        self.inner.kind
    }

    #[track_caller]
    #[inline]
    fn new(kind: Self::Kind) -> Self {
        Self::with_description(kind, String::new())
    }

    #[track_caller]
    #[inline]
    fn with_description<S>(kind: Self::Kind, description: S) -> Self
    where
        S: StaticString,
    {
        Self {
            inner: Arc::new(BoxedData {
                kind,
                metadata: ErrorMetadata::new(description, Some(Location::caller())),
            }),
        }
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> ErrorContext for Diagnostic<K> {
    #[track_caller]
    #[inline]
    fn with_context<C, F>(mut self, context: F) -> Self
    where
        C: Context,
        F: FnOnce() -> C,
    {
        Arc::make_mut(&mut self.inner)
            .metadata
            .context
            .push(ContextEntry {
                data: Arc::new(context()),
                location: Location::caller(),
            });
        self
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> From<K> for Diagnostic<K> {
    #[inline]
    fn from(orig: K) -> Self {
        Self {
            inner: Arc::new(BoxedData {
                kind: orig,
                metadata: ErrorMetadata::new("", None),
            }),
        }
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> From<Described<K>> for Diagnostic<K> {
    #[track_caller]
    #[inline]
    fn from(orig: Described<K>) -> Self {
        Self {
            inner: Arc::new(BoxedData {
                kind: orig.error_kind(),
                metadata: ErrorMetadata::new(orig.into_description(), None),
            }),
        }
    }
}

// -----------------------------------------------------------------------------
impl<K: ErrorKind> From<Located<K>> for Diagnostic<K> {
    #[track_caller]
    #[inline]
    fn from(orig: Located<K>) -> Self {
        Self {
            inner: Arc::new(BoxedData {
                kind: orig.error_kind(),
                metadata: ErrorMetadata::new("", orig.into_location()),
            }),
        }
    }
}
