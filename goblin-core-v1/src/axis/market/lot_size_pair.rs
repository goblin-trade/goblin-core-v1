use crate::{
    axis::leg::{leg_matcher::LegMatcher, leg_quantities::LegQuantities, Base, Pair, Quote},
    for_axes,
    goblin_error::GoblinError,
    require,
};

pub type LotSizePair =
    Pair<<Base as LegQuantities>::LotsPerUnit, <Quote as LegQuantities>::LotsPerUnit>;

impl LotSizePair {
    pub fn validate(&self) -> Result<(), GoblinError> {
        for_axes!(|In| self.validate_leg::<In>()?);
        Ok(())
    }

    fn validate_leg<In: LegMatcher>(&self) -> Result<(), GoblinError> {
        let lot_size = In::get(self);
        require!(
            In::lots_per_unit_valid(lot_size),
            GoblinError::InvalidLotSize
        );
        Ok(())
    }
}
