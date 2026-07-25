use crate::{
    axis::leg::{leg_constants::LegConstants, Base},
    quantities::{BaseAtomsPerBaseUnit, Position, ATOMS_PER_UNIT},
};

impl LegConstants for Base {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = BaseAtomsPerBaseUnit::new(ATOMS_PER_UNIT.inner);
    const DEFAULT_PRICE_LIMIT: Position = Position::ZERO;
}
