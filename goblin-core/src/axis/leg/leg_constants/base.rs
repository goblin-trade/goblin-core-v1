use crate::{
    axis::leg::{Base, LegConstants},
    quantities::{ATOMS_PER_UNIT, BaseAtomsPerBaseUnit, FullPosU32, FullPositionU32, U32Variant},
};

impl LegConstants for Base {
    const ATOMS_PER_UNIT: U32Variant<Self::AtomsPerUnit> =
        U32Variant::<BaseAtomsPerBaseUnit>::new(ATOMS_PER_UNIT.inner as u32);
    const DEFAULT_PRICE_LIMIT: FullPosU32 = FullPosU32::ZERO;
}
