use crate::{
    axis::{
        caller::{CallerEnum, CallerMarker},
        token::{
            token_marker::{TokenData, TokenMarker},
            ETHStub, ETH,
        },
        token_marker::eth::ETH_STORE_HASH_LIST,
        update::UpdateMarker,
        HardcodedCallerList,
    },
    goblin_error::GoblinError,
    quantities::{ETHAtoms, UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    state::{Preimage, SlotKey, StorePreimage},
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

    fn get_store_hash<CM: CallerMarker>(
        preimage: &StorePreimage<Self>,
        _token_index: Self::TokenIndex,
        caller_index: CM::CallerIndex,
    ) -> SlotKey<StorePreimage<Self>> {
        // TODO fix
        // HardcodedCaller is guaranteed to have ETH
        //
        // TODO make call on CallMarker trait instead of matching enum
        match CM::VARIANT {
            CallerEnum::HardcodedCaller => match HardcodedCallerList::index(&preimage.trader) {
                Some(idx) => ETH_STORE_HASH_LIST[idx.inner],
                None => preimage.hash(),
            },
            CallerEnum::CustomCaller => preimage.hash(),
        }
    }

    ////////////////////

    fn get_stored_decimals(
        _token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        Ok(ETHStub)
    }
}
