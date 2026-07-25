use crate::axis::leg::{leg_quantities::LegQuantities, Base, Pair, Quote};

// TODO validate LotSizes.
// Currently LegValidator::lots_per_unit_valid() is unused
pub type LotSizePair =
    Pair<<Base as LegQuantities>::LotsPerUnit, <Quote as LegQuantities>::LotsPerUnit>;
