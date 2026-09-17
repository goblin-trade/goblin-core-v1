use crate::{
    axis::leg::{Base, Pair, Quote, leg_validator::LegValidator},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit},
    types::StoreReader,
};

/// Lot sizes for the base and quote legs.
///
/// `I` is `u64` for internal math and `u32` for the compact wire form.
pub type LotSizePair<I> = Pair<BaseLotsPerBaseUnit<I>, QuoteLotsPerQuoteUnit<I>>;

impl LotSizePair<u32> {
    /// Validate the lot sizes in 32 bit form, as they appear on the wire.
    pub const fn valid(&self) -> bool {
        let base_lot_size = Base::get(self);
        let base_valid = Base::lots_per_unit_valid(base_lot_size);

        let quote_lot_size = Quote::get(self);
        let quote_valid = Quote::lots_per_unit_valid(quote_lot_size);

        base_valid && quote_valid
    }
}
