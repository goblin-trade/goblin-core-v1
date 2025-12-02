use crate::{
    tokens::{ERC20Token, TokenIndex},
    types::Address,
};

/// Key to index Maker delta for a given token index
pub struct TokenMakerDeltaKey<T: ERC20Token> {
    pub maker: Address,
    pub token_index: TokenIndex<T>,
}
