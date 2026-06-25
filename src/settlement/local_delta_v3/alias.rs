use crate::{
    axis::leg::{leg_quantities::LegQuantities, Base, Pair, Quote},
    settlement::ConstZero,
};

pub type DeltaLotsPair =
    Pair<<Base as LegQuantities>::DeltaLots, <Quote as LegQuantities>::DeltaLots>;

impl ConstZero for DeltaLotsPair {
    const ZEROED: Self = Pair::new(
        <Base as LegQuantities>::DeltaLots::ZEROED,
        <Quote as LegQuantities>::DeltaLots::ZEROED,
    );
}
