use crate::{
    axis::{
        caller::HardcodedCaller,
        token::{
            ETH, ETHStub,
            token_list::eth::ETH_STORE_LIST,
            token_marker::{TokenData, TokenMarker},
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::UnsidedAtomsPerLot,
    settlement::UpdateParams,
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

impl TokenMarker for ETH {
    fn get_global_deposit(
        _local_deposit: Self::LocalDeposit,
        _atoms_per_lot: UnsidedAtomsPerLot<i64>,
    ) -> Result<Self::GlobalDeposit, GoblinError> {
        Ok(ETHStub)
    }

    fn update<'a, UM: UpdateMarker>(
        update_params: UpdateParams<'a, Self, UM>,
    ) -> Result<(), GoblinError> {
        update_params.update_eth()
    }

    fn get_hardcoded_store_hash(
        indexed_preimage: &IndexedPreimage<HardcodedCaller, Self>,
    ) -> SlotKey<StorePreimage<Self>> {
        ETH_STORE_LIST[&indexed_preimage.store_key_index]
    }

    ////////////////////

    fn get_stored_decimals(
        _token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        Ok(ETHStub)
    }
}
