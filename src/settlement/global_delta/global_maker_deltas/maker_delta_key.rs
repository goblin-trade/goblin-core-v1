use crate::{token::TokenMarker, types::Address};

/// Key to lookup delta of a maker for a given token
#[derive(Clone, Copy, PartialEq)]
pub struct MakerDeltaKey<T: TokenMarker> {
    pub maker: Address,
    pub token_index: T::TokenIndex,
}
