use crate::{
    axis::leg::{leg_constants::LegConstants, Base},
    quantities::{BaseAtomsPerBaseUnit, QuantityOps, Ticks},
};

impl LegConstants for Base {
    const ATOMS_PER_UNIT: Self::AtomsPerUnit = BaseAtomsPerBaseUnit::new(1_000_000);
    const DEFAULT_PRICE_LIMIT: Ticks = Ticks::ZERO;
}
