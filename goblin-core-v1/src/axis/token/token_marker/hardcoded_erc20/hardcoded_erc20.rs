use crate::{
    axis::{
        caller::{CallerEnum, CallerMarker},
        token::{
            token_list::{hardcoded_erc20::HARDCODED_ERC20_COUNT, HARDCODED_ERC20_STORE_LIST},
            token_marker::{TokenData, TokenMarker},
            HardcodedERC20,
        },
        update::UpdateMarker,
        HardcodedCallerList,
    },
    goblin_error::GoblinError,
    quantities::{UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    settlement::global_delta::TransferERC20,
    state::{IndexedPreimage, Preimage, SlotKey, StorePreimage},
    types::Address,
};

impl TokenMarker for HardcodedERC20 {
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
        match CM::VARIANT {
            // TODO remove. Accept caller index in param
            CallerEnum::HardcodedCaller => match HardcodedCallerList::index(&preimage.trader) {
                Some(caller_idx) => {
                    if token_index.inner < HARDCODED_ERC20_COUNT {
                        // TODO use Index trait to get inner
                        HARDCODED_ERC20_STORE_LIST.inner[caller_idx.inner][token_index.inner]
                    } else {
                        preimage.hash()
                    }
                }
                None => preimage.hash(),
            },
            CallerEnum::CustomCaller => preimage.hash(),
        }
    }

    ///////////

    fn get_stored_decimals(
        token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        Ok(token_data.decimals)
    }
}
