use crate::{
    axis::leg::{Base, Pair, Quote, leg_quantities::LegQuantities, leg_validator::LegValidator},
    for_axes,
    goblin_error::GoblinError,
    require,
    types::StoreReader,
};

pub type LotSizePair =
    Pair<<Base as LegQuantities>::LotsPerUnit, <Quote as LegQuantities>::LotsPerUnit>;

impl LotSizePair {
    pub const fn validate(&self) -> Result<(), GoblinError> {
        for_axes!(In => {
            let lot_size = In::get(self);
            require!(
                In::lots_per_unit_valid(lot_size),
                GoblinError::InvalidLotSize
            );
        });

        Ok(())
    }
}
