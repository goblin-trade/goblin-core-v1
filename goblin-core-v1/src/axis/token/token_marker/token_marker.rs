use crate::{
    axis::{
        token::{token_marker::TokenData, token_reader::TokenReader, TokenEnum},
        update::UpdateMarker,
        HardcodedCaller,
    },
    axis_helpers::AxisMarker,
    goblin_error::GoblinError,
    input_processor::CallerAddresses,
    quantities::{UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

pub trait TokenMarker: 'static + TokenReader + AxisMarker<Enum = TokenEnum> {
    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit;

    // Transfer the token in or out, based on UM
    fn update<UM: UpdateMarker>(
        deposit: UnsidedAtoms,
        token_address: &Self::TokenAddress,
        decimals: Self::StoredDecimals,
        caller_addresses: CallerAddresses,
    ) -> Result<(), GoblinError>;

    fn get_hardcoded_store_hash(
        indexed_preimage: &IndexedPreimage<HardcodedCaller, Self>,
    ) -> SlotKey<StorePreimage<Self>>;

    ////////////////////////

    /// Try to obtain stored decimals
    ///
    /// * ETH: Stub value
    /// * Hardcoded: Use the hardcoded decimals
    /// * Custom: Read from Hostio
    ///
    fn get_stored_decimals(
        token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError>;
}
