use crate::{
    goblin_error::GoblinError,
    settlement::local_delta::Deposits,
    token::{CustomERC20Data, TokenMarker},
    types::LegMatcher,
};

#[derive(Clone, Copy, Default, PartialEq)]
pub struct ETH;

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;
    // type TupleMarker = Self;

    type TokenIndex = ();
    type Address = ();
    type Deposit = ();

    fn token_index_to_address(
        _token_index: Self::TokenIndex,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        Ok(())
    }

    // Stub. ETH cannot be deposited.
    fn set_deposit<In: LegMatcher>(_deposits: &mut Deposits, _deposit_amount: Self::Deposit) {}
}
