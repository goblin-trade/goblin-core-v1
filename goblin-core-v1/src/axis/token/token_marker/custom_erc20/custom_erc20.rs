use crate::{
    axis::{
        caller::CallerMarker,
        token::{
            token_marker::{TokenData, TokenMarker},
            CustomERC20,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    hostio::erc20_hostio,
    quantities::{UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    settlement::global_delta::TransferERC20,
    state::{IndexedPreimage, Preimage, SlotKey, StorePreimage},
    types::Address,
};

impl TokenMarker for CustomERC20 {
    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        local_deposit * atoms_per_lot
    }

    fn update<UM: UpdateMarker>(
        deposit: UnsidedAtoms,
        trader: &Address,
        token_address: &Self::TokenAddress,
        decimals: Self::StoredDecimals,
    ) -> Result<(), GoblinError> {
        TransferERC20::<UM>::new(deposit, trader, token_address, decimals).dispatch()
    }

    fn get_store_hash<CM: CallerMarker>(
        indexed_preimage: &IndexedPreimage<CM, Self>,
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
