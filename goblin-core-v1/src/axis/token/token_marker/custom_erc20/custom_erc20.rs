use crate::{
    axis::{
        token::{
            token_marker::{TokenData, TokenMarker},
            CustomERC20,
        },
        update::UpdateMarker,
        HardcodedCaller,
    },
    goblin_error::GoblinError,
    hostio::erc20_hostio,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::global_delta::UpdateParams,
    state::{IndexedPreimage, Preimage, SlotKey, StorePreimage},
};

impl TokenMarker for CustomERC20 {
    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        local_deposit * atoms_per_lot
    }

    fn update<'a, UM: UpdateMarker>(
        update_params: UpdateParams<'a, Self, UM>,
    ) -> Result<(), GoblinError> {
        update_params.update_erc20()
    }

    fn get_hardcoded_store_hash(
        indexed_preimage: &IndexedPreimage<HardcodedCaller, Self>,
    ) -> SlotKey<StorePreimage<Self>> {
        indexed_preimage.preimage.hash()
    }

    ///////

    fn get_stored_decimals(
        token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        erc20_hostio::decimals(&token_data.address)
    }
}
