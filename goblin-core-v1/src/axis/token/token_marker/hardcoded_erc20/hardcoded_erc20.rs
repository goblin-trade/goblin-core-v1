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
    input_processor::CallerAddresses,
    quantities::{UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    settlement::global_delta::TransferERC20,
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

impl TokenMarker for HardcodedERC20 {
    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        local_deposit * atoms_per_lot
    }

    fn update<UM: UpdateMarker>(
        token_address: &Self::TokenAddress,
        caller_addresses: CallerAddresses,
        deposit: UnsidedAtoms,
        decimals: Self::StoredDecimals,
    ) -> Result<(), GoblinError> {
        TransferERC20::<UM>::new(token_address, caller_addresses, deposit, decimals).update_erc20()
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
