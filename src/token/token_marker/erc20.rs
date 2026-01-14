use crate::{
    goblin_error::GoblinError,
    market::MarketVariant,
    quantities::DeltaAtoms,
    token::{CustomERC20Store, TokenMarker},
    types::Address,
};

#[derive(Clone, Copy, Default)]
pub struct ERC20;

impl TokenMarker for ERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex<M: MarketVariant> = M::TokenIndex;
    type Address = Address;
    type Deposit = DeltaAtoms;

    fn token_index_to_address<M: MarketVariant>(
        token_index: Self::TokenIndex<M>,
        custom_erc20_list: &[CustomERC20Store],
    ) -> Result<Self::Address, GoblinError> {
        // TODO resolve multiple levels
        M::token_index_to_address(token_index, custom_erc20_list)
    }
}
