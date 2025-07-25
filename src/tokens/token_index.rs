use crate::{
    goblin_error::GoblinError,
    tokens::{ERC20Token, Token, HARDCODED_TOKENS},
    types::Address,
};

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq)]
pub struct TokenIndex(pub u8);

impl TokenIndex {
    const ETH: u8 = 255;

    pub fn is_eth(&self) -> bool {
        self.0 == Self::ETH
    }

    /// Obtain the token corresponding to the token index.
    /// Token can be ETH or an ERC20
    ///
    /// Token index to token lookup happens in two places
    /// * To obtain slot keys for a market
    /// * In the settlement phase for token transfers and slot updates.
    ///
    /// In the intermediary stages, it is possible that an invalid token index is passed to ERC20Delta.
    /// However this is caught during settlement.
    ///
    /// We do not store token address inside ERC20Delta. While this allows us to pass the address
    /// found in the market key generation phase, it means we need to find the address when converting
    /// erc20_delta_input_list to erc20_delta_list. We cannot avoid duplicate lookup either way.
    pub fn to_token(&self, custom_erc20_list: &[Address]) -> Result<Token, GoblinError> {
        let index = self.0 as usize;

        if index < custom_erc20_list.len() {
            let token = custom_erc20_list[index];
            Ok(Token::ERC20(ERC20Token::Custom(token)))
        } else if index > 127 && index < (127 + HARDCODED_TOKENS.len()) {
            let token = HARDCODED_TOKENS[index - 127];
            Ok(Token::ERC20(ERC20Token::Hardcoded(token)))
        } else if self.is_eth() {
            Ok(Token::Eth)
        } else {
            Err(GoblinError::NoTokenAtIndex)
        }
    }
}
