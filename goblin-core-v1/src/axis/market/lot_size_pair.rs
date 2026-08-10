use crate::{
    axis::leg::{leg_quantities::LegQuantities, leg_validator::LegValidator, Base, Pair, Quote},
    for_axes,
    goblin_error::GoblinError,
    require,
    types::StoreReader,
};

pub type LotSizePair =
    Pair<<Base as LegQuantities>::LotsPerUnit, <Quote as LegQuantities>::LotsPerUnit>;

impl LotSizePair {
    pub fn validate(&self) -> Result<(), GoblinError> {
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
