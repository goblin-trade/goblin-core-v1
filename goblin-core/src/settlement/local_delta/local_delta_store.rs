use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_quantities::LegQuantities},
        update::{Decrease, Increase, UpdateMarker},
    },
    goblin_error::GoblinError,
};

pub trait LocalDeltaStore {
    fn add_leg<UM: UpdateMarker, In: LegMatcher>(
        &mut self,
        lots: In::Lots,
    ) -> Result<(), GoblinError>;

    fn add<In: LegMatcher>(
        &mut self,
        lots: In::Lots,
        lots_opposite: <In::Opposite as LegQuantities>::Lots,
    ) -> Result<(), GoblinError> {
        self.add_leg::<Decrease, In>(lots)?;
        self.add_leg::<Increase, In::Opposite>(lots_opposite)?;

        Ok(())
    }
}
