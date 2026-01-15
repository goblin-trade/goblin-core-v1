use crate::{
    goblin_error::GoblinError,
    market::MarketVariant,
    quantities::DeltaAtoms,
    token::{AddressMapper, CustomERC20Data, TokenMarker},
    types::Address,
};

#[derive(Clone, Copy, Default)]
pub struct ERC20;

impl TokenMarker for ERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex<M: MarketVariant> = M::MarketERC20Index;
    type Address = Address;
    type Deposit = DeltaAtoms;

    fn token_index_to_address<M: MarketVariant>(
        token_index: Self::TokenIndex<M>,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        // Get address from hardcoded or custom erc20 list depending on token index
        token_index.address(custom_erc20_list)
    }
}
