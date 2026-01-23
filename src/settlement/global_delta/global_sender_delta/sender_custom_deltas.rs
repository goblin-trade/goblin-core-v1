use crate::settlement::global_delta::ERC20Delta;

pub const MAX_CUSTOM_DELTAS: usize = 8;

/// Deltas of custom tokens.
///
/// # Safety
///
/// TokenIndex<CustomToken>::new() ensures that index is within MAX_CUSTOM_DELTAS bounds.
/// Therefore lookups are safe.
pub type SenderCustomDeltas = [ERC20Delta; MAX_CUSTOM_DELTAS];
