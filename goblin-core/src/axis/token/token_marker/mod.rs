pub mod custom_erc20;
pub mod eth;
pub mod hardcoded_erc20;
pub mod token_data;

pub use custom_erc20::*;
pub use hardcoded_erc20::*;
pub use token_data::*;

use super::{TokenEnum, token_reader::TokenReader};
use crate::{
    axis::{caller::HardcodedCaller, update::UpdateMarker},
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
