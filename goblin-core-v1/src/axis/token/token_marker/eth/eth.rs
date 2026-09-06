use crate::{
    axis::{
        caller::{CallerEnum, CallerMarker},
        token::{
            token_list::eth::ETH_STORE_LIST,
            token_marker::{TokenData, TokenMarker},
            ETHStub, ETH,
        },
        update::UpdateMarker,
        HardcodedCallerList,
    },
    goblin_error::GoblinError,
    quantities::{ETHAtoms, UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    state::{IndexedPreimage, Preimage, SlotKey, StorePreimage},
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
        indexed_preimage: &IndexedPreimage<CM, Self>,
    ) -> SlotKey<StorePreimage<Self>> {
        // TODO need trait to cover both HardcodedCaller and CustomCaller
        //
        // Better alternative- move get_store_hash() on CallMarker itself
        match CM::VARIANT {
            CallerEnum::HardcodedCaller => ETH_STORE_LIST[indexed_preimage],
            CallerEnum::CustomCaller => indexed_preimage.preimage.hash(),
        }

        // TODO fix
        // HardcodedCaller is guaranteed to have ETH
        //
        // TODO make call on CallMarker trait instead of matching enum
        // match CM::VARIANT {
        //     CallerEnum::HardcodedCaller => match HardcodedCallerList::index(&preimage.trader) {
        //         Some(idx) => ETH_STORE_LIST.inner[idx.inner],
        //         None => preimage.hash(),
        //     },
        //     CallerEnum::CustomCaller => preimage.hash(),
        // }
    }

    ////////////////////

    fn get_stored_decimals(
        _token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        Ok(ETHStub)
    }
}
