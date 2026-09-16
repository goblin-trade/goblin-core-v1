use crate::{
    axis::leg::{Base, LegConstants},
    quantities::{ATOMS_PER_UNIT, BaseAtomsPerBaseUnit, FullPosU32, FullPositionU32},
};

impl LegConstants for Base {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = BaseAtomsPerBaseUnit::new(ATOMS_PER_UNIT.inner);
    const DEFAULT_PRICE_LIMIT: FullPosU32 = FullPosU32::ZERO;
}
