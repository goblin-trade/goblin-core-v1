use crate::{
    goblin_error::GoblinError,
    require,
    tokens::{ERC20Token, Token, TokenIndex},
    types::Address,
};

pub struct ERC20TokenPair {
    pub base_token: ERC20Token,
    pub quote_token: ERC20Token,
}

pub enum ValidatedTokenPair {
    ERC20ERC20(ERC20TokenPair),
    ETHERC20(ERC20Token),
    ERC20ETH(ERC20Token),
}

impl ValidatedTokenPair {
    pub fn new(
        base_token_index: TokenIndex,
        quote_token_index: TokenIndex,
        custom_erc20_list: &[Address],
    ) -> Result<Self, GoblinError> {
        require!(
            base_token_index != quote_token_index,
            GoblinError::InvalidTokenPair
        );

        let base_token = base_token_index.to_token(custom_erc20_list)?;
        let quote_token = quote_token_index.to_token(custom_erc20_list)?;

        match (base_token, quote_token) {
            (Token::ERC20(base_token), Token::ERC20(quote_token)) => {
                Ok(ValidatedTokenPair::ERC20ERC20(ERC20TokenPair {
                    base_token,
                    quote_token,
                }))
            }
            (Token::Eth, Token::ERC20(quote_erc20)) => {
                Ok(ValidatedTokenPair::ETHERC20(quote_erc20))
            }
            (Token::ERC20(base_erc20), Token::Eth) => Ok(ValidatedTokenPair::ERC20ETH(base_erc20)),
            (Token::Eth, Token::Eth) => {
                // This case is unreachable due to the require! check above
                Err(GoblinError::InvalidTokenPair)
            }
        }
    }

    pub const fn discriminator(&self) -> u8 {
        match self {
            ValidatedTokenPair::ERC20ERC20(_) => 3,
            ValidatedTokenPair::ETHERC20(_) => 4,
            ValidatedTokenPair::ERC20ETH(_) => 5,
        }
    }
}
