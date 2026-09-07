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
        token_address: &Self::TokenAddress,
        decimals: Self::StoredDecimals,
        caller_addresses: CallerAddresses,
    ) -> Result<(), GoblinError> {
        let amount = ETHAtoms::try_from(deposit)?;
        UM::update_eth(trader, &amount)
        // TransferERC20::<UM>::new(deposit, decimals, token_address, caller_addresses).dispatch()
    }

    // fn update<UM: UpdateMarker>(
    //     deposit: UnsidedAtoms,
    //     trader: &Address,
    //     _token_address: &Self::TokenAddress,
    //     _decimals: Self::StoredDecimals,
    // ) -> Result<(), GoblinError> {
    //     let amount = ETHAtoms::try_from(deposit)?;
    //     UM::update_eth(trader, &amount)
    // }

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
