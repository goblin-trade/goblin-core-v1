use crate::{
    goblin_error::GoblinError,
    tokens::{ERC20Token, Token, HARDCODED_TOKENS},
    types::Address,
};

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq)]
pub struct TokenIndex(pub u8);

impl TokenIndex {
    pub fn is_eth(&self) -> bool {
        self.0 == 255
    }

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
