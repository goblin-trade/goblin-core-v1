use crate::{
    axis::leg::{Base, LegConstants},
    quantities::{ATOMS_PER_UNIT, BaseAtomsPerBaseUnit, FullPosition, PositionV2},
};

impl LegConstants for Base {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = BaseAtomsPerBaseUnit::new(ATOMS_PER_UNIT.inner);
    const DEFAULT_PRICE_LIMIT: PositionV2 = PositionV2::ZERO;
}
