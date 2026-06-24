use crate::axis::leg::{leg_quantities::LegQuantities, Base, Pair, Quote};

pub type DeltaLotsPair =
    Pair<<Base as LegQuantities>::DeltaLots, <Quote as LegQuantities>::DeltaLots>;
