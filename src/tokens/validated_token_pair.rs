use crate::{
    goblin_error::GoblinError,
    markets::TokenIndexPair,
    require,
    tokens::{ERC20Token, Token},
    types::{Address, Pair},
};

pub type ERC20TokenPair = Pair<ERC20Token, ERC20Token>;

pub enum ValidatedTokenPair {
    ERC20ERC20(ERC20TokenPair),
    ETHERC20(ERC20Token),
    ERC20ETH(ERC20Token),
}

impl ValidatedTokenPair {
    pub fn new(
        token_index_pair: TokenIndexPair,
        custom_erc20_list: &[Address],
    ) -> Result<Self, GoblinError> {
        require!(
            token_index_pair.base != token_index_pair.quote,
            GoblinError::InvalidTokenPair
        );

        let base_token = token_index_pair.base.to_token(custom_erc20_list)?;
        let quote_token = token_index_pair.quote.to_token(custom_erc20_list)?;

        match (base_token, quote_token) {
            (Token::ERC20(base_token), Token::ERC20(quote_token)) => {
                Ok(ValidatedTokenPair::ERC20ERC20(ERC20TokenPair {
                    base: base_token,
                    quote: quote_token,
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
