use crate::{
    goblin_error::GoblinError,
    tokens::{ERC20Token, Token, HARDCODED_TOKENS},
    types::Address,
};

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq)]
pub struct TokenIndex(pub u8);

impl TokenIndex {
    pub const ETH: Self = TokenIndex(255);

    /// Obtain the token corresponding to the token index.
    /// Token can be ETH or an ERC20
    ///
    /// Token index to token lookup happens in two places
    /// * To obtain slot keys for a market
    /// * In the settlement phase for token transfers and slot updates.
    ///
    /// This is still cheap because we're looking up values for an index within a slice.
    /// There is no need to store token address in ERC20Delta
    ///
    /// In the intermediary stages, it is possible that an invalid token index is passed to ERC20Delta.
    /// However this is caught during settlement.
    pub fn to_token(&self, custom_erc20_list: &[Address]) -> Result<Token, GoblinError> {
        if *self == Self::ETH {
            return Ok(Token::Eth);
        }

        let index = self.0 as usize;

        if index < custom_erc20_list.len() {
            let token = custom_erc20_list[index];
            Ok(Token::ERC20(ERC20Token::Custom(token)))
        } else if index > 127 && index < (127 + HARDCODED_TOKENS.len()) {
            let token = HARDCODED_TOKENS[index - 127];
            Ok(Token::ERC20(ERC20Token::Hardcoded(token)))
        } else {
            Err(GoblinError::NoTokenAtIndex)
        }
    }
}
