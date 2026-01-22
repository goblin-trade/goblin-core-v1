use crate::{
    goblin_error::GoblinError,
    quantities::DeltaAtoms,
    settlement::local_delta::Deposits,
    token::{CustomERC20, CustomERC20Data, CustomERC20Index, TokenMarker, ERC20},
    types::{Address, Base, LegMatcher, Quote, TupleMarker, TupleReader},
};

impl TokenMarker for CustomERC20 {
    const DISCRIMINATOR: u8 = 2;
    // type TupleMarker = ERC20;

    type TokenIndex = CustomERC20Index;

    type Address = Address;

    type Deposit = <Self::TupleMarker as TupleMarker>::Deposit;
    // type Deposit = DeltaAtoms;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        let data = custom_erc20_list
            .get(token_index.0)
            .ok_or(GoblinError::InvalidCustomTokenIndex)?;
        Ok(data.address)
    }

    fn set_deposit<In>(deposits: &mut Deposits, deposit_amount: Self::Deposit)
    where
        In: LegMatcher + TupleReader<DeltaAtoms, DeltaAtoms, (Base, Quote), Result = DeltaAtoms>,
    {
        *In::get_leg_mut(deposits) = deposit_amount;
    }
}
