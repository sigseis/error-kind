#![doc = include_str!("README.md")]
// =============================================================================
// CRATE CONFIG
// =============================================================================

// -----------------------------------------------------------------------------
#![allow(clippy::module_inception)] // Common pattern for module organization.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "backtrace"), no_std)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![forbid(unsafe_code)]

// =============================================================================
// CRATES
// =============================================================================

// -----------------------------------------------------------------------------
#[cfg(feature = "alloc")]
extern crate alloc as std_alloc;

// -----------------------------------------------------------------------------
#[cfg(test)]
extern crate self as error_kind;

// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
pub mod details;
pub mod envelopes;
mod macros;
mod traits;

// =============================================================================
// RE-EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
pub use traits::*;

// -----------------------------------------------------------------------------
// These are for *this crate's* macro use only - DO NOT TOUCH! DO NOT DEPEND ON!
// -----------------------------------------------------------------------------
#[doc(hidden)]
pub mod __private {
    pub use ::core::format_args;
    #[cfg(feature = "alloc")]
    pub use ::std_alloc::format;
}

// -----------------------------------------------------------------------------
/// Automatically derive the [`Error`] trait
///
#[cfg(feature = "macros")]
#[doc(inline)]
pub use error_kind_macros::Error;

// -----------------------------------------------------------------------------
/// Automatically derive the [`ErrorDomain`] trait
///
#[cfg(feature = "macros")]
#[doc(inline)]
pub use error_kind_macros::ErrorDomain;

// -----------------------------------------------------------------------------
/// Automatically derive the [`ErrorKind`] trait
///
#[cfg(feature = "macros")]
#[doc(inline)]
pub use error_kind_macros::ErrorKind;

// -----------------------------------------------------------------------------
#[cfg(feature = "macros")]
#[doc = include_str!("macros/error.md")]
pub use error_kind_macros::error;
