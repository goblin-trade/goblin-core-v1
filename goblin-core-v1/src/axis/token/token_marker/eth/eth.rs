use goblin_macros::const_keccak256;

use crate::{
    axis::{
        caller::{CallerEnum, CallerMarker, HardcodedCaller, HARDCODED_CALLER_COUNT},
        token::{
            token_marker::{TokenData, TokenMarker},
            ETHStub, ETH,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::{ETHAtoms, UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    state::{Preimage, SlotKey, StorePreimage},
    types::Address,
};

pub const ETH_STORE_HASHES: [SlotKey<StorePreimage<ETH>>; HARDCODED_CALLER_COUNT] = [
    // Caller 0 (0x0)
    SlotKey::new(const_keccak256!(2u8, [0u8; 20])),
    // Caller 1 (0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E)
    SlotKey::new(const_keccak256!(
        2u8,
        [
            0x3f, 0x1e, 0xae, 0x7d, 0x46, 0xd8, 0x8f, 0x08, 0xfc, 0x2f, 0x8e, 0xd2, 0x7f, 0xcb,
            0x2a, 0xb1, 0x83, 0xeb, 0x2d, 0x0e
        ]
    )),
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
