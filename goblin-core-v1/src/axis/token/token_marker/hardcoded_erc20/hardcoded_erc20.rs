use crate::{
    axis::{
        token::{
            token_list::HARDCODED_ERC20_STORE_LIST,
            token_marker::{TokenData, TokenMarker},
            HardcodedERC20,
        },
        update::UpdateMarker,
        HardcodedCaller,
    },
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::global_delta::UpdateParams,
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

impl TokenMarker for HardcodedERC20 {
    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Result<Self::GlobalDeposit, GoblinError> {
        local_deposit
            .checked_mul(atoms_per_lot)
            .ok_or(GoblinError::Overflow)
    }

    fn update<'a, UM: UpdateMarker>(
        update_params: UpdateParams<'a, Self, UM>,
    ) -> Result<(), GoblinError> {
        Ok(())
        // TODO fix problem here. Problem in ETH and HardcodedERC20 version
        // update_params.update_erc20()
    }

    fn get_hardcoded_store_hash(
        indexed_preimage: &IndexedPreimage<HardcodedCaller, Self>,
    ) -> SlotKey<StorePreimage<Self>> {
        HARDCODED_ERC20_STORE_LIST[&indexed_preimage.store_key_index]
    }

    ///////////

    fn get_stored_decimals(
        token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        Ok(token_data.decimals)
    }
}
