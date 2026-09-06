use crate::{
    axis::{
        caller::CallerMarker,
        token::{token_marker::TokenData, token_reader::TokenReader, TokenEnum},
        update::{UpdateEnum, UpdateMarker},
    },
    axis_helpers::AxisMarker,
    goblin_error::GoblinError,
    match_axes,
    quantities::{IntoAbs, UnsidedAtoms, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot},
    state::{indexed_preimage, IndexedPreimage, SlotKey, StorePreimage},
    types::Address,
};

pub trait TokenMarker: 'static + TokenReader + AxisMarker<Enum = TokenEnum> {
    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit;

    fn transfer(
        net_deposit: UnsidedDeltaAtoms,
        trader: &Address,
        token_address: &Self::TokenAddress,
        decimals: Self::StoredDecimals,
    ) -> Result<(), GoblinError> {
        if net_deposit == UnsidedDeltaAtoms::default() {
            return Ok(());
        }

        let update_enum = UpdateEnum::from(net_deposit);
        let deposit = net_deposit.abs();

        match_axes!(UM = update_enum => {
            Self::update::<UM>(deposit, trader, token_address, decimals)?;
        });

        Ok(())
    }

    // Transfer the token in or out, based on UM
    fn update<UM: UpdateMarker>(
        deposit: UnsidedAtoms,
        trader: &Address,
        token_address: &Self::TokenAddress,
        decimals: Self::StoredDecimals,
    ) -> Result<(), GoblinError>;

    fn get_store_hash<CM: CallerMarker>(
        indexed_preimage: &IndexedPreimage<CM, Self>,
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
