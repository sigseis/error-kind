// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
mod context;
mod error;
mod error_context;
mod error_domain;
mod error_kind;
mod option_ext;
mod result_ext;
mod trivial_error;

// =============================================================================
// RE-EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
pub use context::*;
pub use error::*;
pub use error_context::*;
pub use error_domain::*;
pub use error_kind::*;
pub use option_ext::*;
pub use result_ext::*;
pub use trivial_error::*;
