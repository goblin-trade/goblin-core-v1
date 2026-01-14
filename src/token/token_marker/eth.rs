use crate::{
    goblin_error::GoblinError,
    market::MarketVariant,
    token::{CustomERC20Store, TokenMarker},
};

#[derive(Clone, Copy, Default)]
pub struct ETH;

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex<M: MarketVariant> = ();
    type Address = ();
    type Deposit = ();

    fn token_index_to_address<M: MarketVariant>(
        _token_index: Self::TokenIndex<M>,
        _custom_erc20_list: &[CustomERC20Store],
    ) -> Result<Self::Address, GoblinError> {
        Ok(())
    }
}
