use crate::{
    axis::leg::{Base, Pair, Quote, leg_quantities::LegQuantities, leg_validator::LegValidator},
    quantities::U32Variant,
    types::StoreReader,
};

pub type LotSizePair =
    Pair<<Base as LegQuantities>::LotsPerUnit, <Quote as LegQuantities>::LotsPerUnit>;

pub type LotSizePairU32 = Pair<
    U32Variant<<Base as LegQuantities>::LotsPerUnit>,
    U32Variant<<Quote as LegQuantities>::LotsPerUnit>,
>;

impl LotSizePairU32 {
    pub fn gg(&self) {
        let zz = LotSizePair::from(self);
    }
}

impl LotSizePair {
    pub const fn valid(&self) -> bool {
        let base_lot_size = Base::get(self);
        let base_valid = Base::lots_per_unit_valid(base_lot_size);

        let quote_lot_size = Quote::get(self);
        let quote_valid = Quote::lots_per_unit_valid(quote_lot_size);

        base_valid && quote_valid
    }
}
