use crate::{
    axis::token::CustomERC20,
    settlement::{global_delta::SenderTokenStore, ConstZero},
};

pub const MAX_CUSTOM_DELTAS: usize = 8;

/// Deltas of custom tokens.
///
/// # Safety
///
/// TokenIndex<CustomToken>::new() ensures that index is within MAX_CUSTOM_DELTAS bounds.
/// Therefore lookups are safe.
pub type SenderCustomDeltas = [SenderTokenStore<CustomERC20>; MAX_CUSTOM_DELTAS];

impl ConstZero for SenderCustomDeltas {
    const ZEROED: Self = [SenderTokenStore::ZEROED; MAX_CUSTOM_DELTAS];
}
