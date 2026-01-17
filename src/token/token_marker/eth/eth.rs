use crate::{
    goblin_error::GoblinError,
    token::{CustomERC20Data, TokenMarker},
    types::TupleMarker,
};

#[derive(Clone, Copy, Default)]
pub struct ETH;

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;
    type TupleMarker = Self;

    type TokenIndex = ();
    type Address = ();
    type Deposit = <Self::TupleMarker as TupleMarker>::Deposit;
    // type Deposit = ();

    fn token_index_to_address(
        _token_index: Self::TokenIndex,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        Ok(())
    }
}
