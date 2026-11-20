// =============================================================================
// MODULES
// =============================================================================

// -----------------------------------------------------------------------------
// For these generic, non-trivial types, it's highly recommended that you place
// a bounds on `K` of `ErrorKind`. Things get difficult without this.
// -----------------------------------------------------------------------------
mod bare;
mod bare_conversion;
mod described;
mod described_conversion;
mod diagnostic;
mod diagnostic_conversion;
mod located;
mod located_conversion;
