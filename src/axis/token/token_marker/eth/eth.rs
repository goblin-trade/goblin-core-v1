use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        token::{
            token_marker::{custom_erc20::custom_erc20_data::CustomERC20Data, TokenMarker},
            ETH,
        },
    },
    goblin_error::GoblinError,
    quantities::DeltaAtoms,
    settlement::{global_delta::SenderTokenStore, CheckedAdd, ConstZero, Delta},
    types::StoreReader,
};

impl From<()> for DeltaAtoms {
    fn from(_value: ()) -> Self {
        DeltaAtoms::ZEROED
    }
}

impl From<DeltaAtoms> for () {
    fn from(_value: DeltaAtoms) -> Self {
        ()
    }
}

impl CheckedAdd for () {
    fn checked_add(self, _rhs: Self) -> Option<Self> {
        Some(())
    }
}

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = ();
    type Address = ();
    type Deposit = ();
    // type Delta = EthDelta;

    fn token_index_to_address(
        _token_index: Self::TokenIndex,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        Ok(())
    }

    fn get_global_delta<In>(
        _token_index: Self::TokenIndex,
        delta: &mut Delta,
    ) -> Result<&mut SenderTokenStore<Self>, GoblinError>
    where
        In: LegMatcher,
    {
        let store = Self::get_leg_mut(&mut delta.global.global_sender_delta);
        Ok(store)
    }
}
