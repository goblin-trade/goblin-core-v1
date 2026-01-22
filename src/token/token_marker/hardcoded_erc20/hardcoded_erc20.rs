use crate::{
    goblin_error::GoblinError,
    quantities::DeltaAtoms,
    settlement::local_delta::Deposits,
    token::{CustomERC20Data, HardcodedERC20Index, TokenMarker, HARDCODED_TOKENS},
    types::{Address, Base, LegMatcher, Quote, TupleReader},
};

pub struct HardcodedERC20;

impl TokenMarker for HardcodedERC20 {
    const DISCRIMINATOR: u8 = 1;
    // type TupleMarker = ERC20;

    type TokenIndex = HardcodedERC20Index;

    type Address = Address;

    type Deposit = DeltaAtoms;
    // type Deposit = DeltaAtoms;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        let data = HARDCODED_TOKENS
            .get(token_index.0)
            .ok_or(GoblinError::InvalidHardcodedTokenIndex)?;

        Ok(data.address)
    }

    fn set_deposit<In>(deposits: &mut Deposits, deposit_amount: Self::Deposit)
    where
        In: LegMatcher + TupleReader<DeltaAtoms, DeltaAtoms, (Base, Quote), Result = DeltaAtoms>,
    {
        *In::get_leg_mut(deposits) = deposit_amount;
    }
}
