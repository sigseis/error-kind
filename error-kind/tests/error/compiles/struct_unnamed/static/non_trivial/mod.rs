// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
// For these generic, non-trivial types, it's highly recommended that you place
// a bounds on `K` of `ErrorKind`. Things get difficult without this.
// -----------------------------------------------------------------------------
mod bare;
mod described;
mod diagnostic;
mod located;
