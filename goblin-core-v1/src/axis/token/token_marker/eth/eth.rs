use crate::{
    axis::{
        token::{
            token_marker::{TokenData, TokenMarker},
            ETHStub, ETH,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::{ETHAtoms, UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    types::Address,
};

impl TokenMarker for ETH {
    fn get_global_deposit(
        _local_deposit: Self::LocalDeposit,
        _atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        ETHStub
    }

    fn update<UM: UpdateMarker>(
        deposit: UnsidedAtoms,
        trader: &Address,
        _token_address: &Self::TokenAddress,
        _decimals: Self::StoredDecimals,
    ) -> Result<(), GoblinError> {
        let amount = ETHAtoms::try_from(deposit)?;
        UM::update_eth(trader, &amount)
    }

    ////////////////////

    fn get_stored_decimals(
        _token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        Ok(ETHStub)
    }
}
