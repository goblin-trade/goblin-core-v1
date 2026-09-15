use crate::{
    axis::leg::{Base, LegConstants},
    quantities::{ATOMS_PER_UNIT, BaseAtomsPerBaseUnit, FullPosition, Position},
};

impl LegConstants for Base {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = BaseAtomsPerBaseUnit::new(ATOMS_PER_UNIT.inner);
    const DEFAULT_PRICE_LIMIT: Position = Position::ZERO;
}
