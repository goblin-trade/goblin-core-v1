use crate::{
    axis::{
        token::{
            token_list::eth::ETH_STORE_LIST,
            token_marker::{TokenData, TokenMarker},
            ETHStub, ETH,
        },
        update::UpdateMarker,
        HardcodedCaller,
    },
    goblin_error::GoblinError,
    input_processor::CallerAddresses,
    quantities::{ETHAtoms, UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

impl TokenMarker for ETH {
    fn get_global_deposit(
        _local_deposit: Self::LocalDeposit,
        _atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        ETHStub
    }

    fn update<UM: UpdateMarker>(
        _token_address: &Self::TokenAddress,
        caller_addresses: CallerAddresses,
        deposit: UnsidedAtoms,
        _decimals: Self::StoredDecimals,
    ) -> Result<(), GoblinError> {
        let amount = ETHAtoms::try_from(deposit)?;
        let update_address = UM::update_address(caller_addresses);
        UM::update_eth(update_address, &amount)
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
