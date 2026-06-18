use crate::{axis::token::token_reader::TokenReader, types::Address};

/// Key to lookup delta of a maker for a given token
#[derive(Clone, Copy, PartialEq)]
pub struct MakerDeltaKey<T: TokenReader> {
    pub maker: Address,
    pub token_index: T::TokenIndex,
}
