use crate::{
    quantities::{BaseAtomsPerBaseUnit, QuantityOps, Ticks},
    types::{Base, LegConstants},
};

impl LegConstants for Base {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = BaseAtomsPerBaseUnit::new(1_000_000);
    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::ZERO;
}
