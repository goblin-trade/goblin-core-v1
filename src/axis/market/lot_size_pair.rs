use crate::{
    axis::{
        leg::{leg_quantities::LegQuantities, Base, Pair, Quote, SamePair},
        update::Increase,
    },
    goblin_error::GoblinError,
    quantities::{
        TryIntoUnsidedDelta, UnsideQuantity, UnsidedAtomsPerLot, UnsidedDeltaAtomsPerLot,
        ATOMS_PER_UNIT, DELA_ATOMS_PER_UNIT,
    },
};

// TODO validate LotSizes.
// Currently LegValidator::lots_per_unit_valid() is unused
pub type LotSizePair =
    Pair<<Base as LegQuantities>::LotsPerUnit, <Quote as LegQuantities>::LotsPerUnit>;

// Can we have more generic operations on tuple?
//
// * Multiplying with scalar
// * Checked add with another tuple

impl From<&LotSizePair> for SamePair<UnsidedAtomsPerLot> {
    fn from(value: &LotSizePair) -> Self {
        let base_atoms_per_lot = ATOMS_PER_UNIT / value.0.unsided();
        let quote_atoms_per_lot = ATOMS_PER_UNIT / value.1.unsided();

        Pair::new(base_atoms_per_lot, quote_atoms_per_lot)
    }
}

impl TryFrom<&LotSizePair> for SamePair<UnsidedDeltaAtomsPerLot> {
    type Error = GoblinError;

    fn try_from(value: &LotSizePair) -> Result<Self, Self::Error> {
        let base_lot_size = value.0.try_into_unsided_delta::<Increase>()?;
        let base_delta_atoms_per_lot = DELA_ATOMS_PER_UNIT / base_lot_size;

        let quote_lot_size = value.1.try_into_unsided_delta::<Increase>()?;
        let quote_delta_atoms_per_lot = DELA_ATOMS_PER_UNIT / quote_lot_size;

        Ok(Pair::new(
            base_delta_atoms_per_lot,
            quote_delta_atoms_per_lot,
        ))
    }
}
