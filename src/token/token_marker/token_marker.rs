use crate::{goblin_error::GoblinError, market::MarketVariant, token::CustomERC20Data};

/// Marker class for 'Token'. We have 2 tokens
///
/// 1. ETH
/// 2. ERC20- this has sub variants hardcoded and custom, covered by ERC20Marker
pub trait TokenMarker: Clone + Copy {
    const DISCRIMINATOR: u8;

    /// Index to lookup token address
    type TokenIndex<M: MarketVariant>: Clone + Copy;

    /// Token address
    type Address: Clone + Copy + Sized + Default;

    /// Data type representing pending deposit amount
    type Deposit: Clone + Copy + Default;

    /// Map the token index to address
    ///
    /// This function has 2 variations for TokenMarker and MarketVariant traits
    ///
    /// 1. TokenMarker (ETH / ERC20): Maps ETH to (). If token is ERC20 then
    /// calls MarketVariant::token_index_to_address()
    ///
    /// 2. MarketVariant (Hardcoded / Dynamic): Reads token address from
    /// hardcoded or custom token list
    fn token_index_to_address<M: MarketVariant>(
        token_index: Self::TokenIndex<M>,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError>;
}
