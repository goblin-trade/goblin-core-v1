use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::LotSizePair,
        token::{
            token_marker::{
                custom_erc20::{
                    custom_erc20_data::CustomERC20Data, custom_erc20_index::CustomERC20Index,
                },
                TokenMarker,
            },
            CustomERC20,
        },
    },
    goblin_error::GoblinError,
    quantities::DeltaAtoms,
    settlement::{
        local_delta::Deposits, Delta, SidedSenderDeltaV2, UnsideDelta, UnsidedSenderDeltaV2,
    },
    types::Address,
};

impl TokenMarker for CustomERC20 {
    const DISCRIMINATOR: u8 = 2;

    type TokenIndex = CustomERC20Index;
    type Address = Address;
    type Deposit = DeltaAtoms;

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
        In: LegMatcher,
    {
        *In::get_leg_mut(deposits) = deposit_amount;
    }

    fn add_delta<In>(delta: &mut Delta, lot_size_pair: &LotSizePair)
    where
        In: LegMatcher,
        SidedSenderDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedSenderDeltaV2>,
    {
        todo!()
    }
}
