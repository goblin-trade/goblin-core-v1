use crate::{
    goblin_error::GoblinError,
    quantities::DeltaAtoms,
    token::{CustomERC20, CustomERC20Data, ERC20Index, ERC20Marker, TokenMarker, ERC20},
    types::{Address, TupleMarker},
};

impl TokenMarker for CustomERC20 {
    const DISCRIMINATOR: u8 = 2;
    type TupleMarker = ERC20;

    type TokenIndex = ERC20Index<Self>;

    type Address = Address;

    type Deposit = <Self::TupleMarker as TupleMarker>::Deposit;
    // type Deposit = DeltaAtoms;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        let data = CustomERC20::get_data(token_index, custom_erc20_list)?;
        Ok(data.address)
    }
}
