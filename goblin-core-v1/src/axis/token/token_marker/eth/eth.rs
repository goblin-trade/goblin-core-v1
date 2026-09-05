use crate::{
    axis::{
        caller::{CallerEnum, CallerMarker, HardcodedCaller},
        token::{
            token_marker::{TokenData, TokenMarker},
            ETHStub, ETH,
        },
        update::UpdateMarker,
        HARDCODED_CALLERS,
    },
    goblin_error::GoblinError,
    quantities::{ETHAtoms, UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    state::{Preimage, SlotKey, StorePreimage},
    types::Address,
};

pub const ETH_STORE_HASHES: [SlotKey<StorePreimage<ETH>>; HARDCODED_CALLERS.len()] = [
    StorePreimage {
        trader: HARDCODED_CALLERS[0],
        token_address: ETHStub,
    }
    .const_hash(),
    StorePreimage {
        trader: HARDCODED_CALLERS[1],
        token_address: ETHStub,
    }
    .const_hash(),
];

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
        _token_index: &Self::TokenIndex,
    ) -> SlotKey<StorePreimage<Self>> {
        match CM::VARIANT {
            CallerEnum::HardcodedCaller => {
                match HardcodedCaller::get_caller_index(&preimage.trader) {
                    Some(idx) => ETH_STORE_HASHES[idx],
                    None => preimage.hash(),
                }
            }
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
