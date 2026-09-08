use crate::{
    axis::{
        caller::HardcodedCaller,
        token::{TokenEnum, token_marker::TokenData, token_reader::TokenReader},
        update::UpdateMarker,
    },
    axis_helpers::AxisMarker,
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::UpdateParams,
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

pub trait TokenMarker: 'static + TokenReader + AxisMarker<Enum = TokenEnum> {
    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Result<Self::GlobalDeposit, GoblinError>;

    // Transfer the token in or out, based on UM
    fn update<'a, UM: UpdateMarker>(
        update_params: UpdateParams<'a, Self, UM>,
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
