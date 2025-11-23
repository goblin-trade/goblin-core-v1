use crate::{
    markets::{MarketVariant, PairShape},
    quantities::QuoteLotsPerBaseUnitPerTick,
    types::{Base, LegMarker, Pair, Quote},
};

pub type LotSizePair = Pair<<Base as LegMarker>::LotsPerUnit, <Quote as LegMarker>::LotsPerUnit>;

pub struct CommonMarket<M: MarketVariant, P: PairShape> {
    /// The token pair, parameterized by shape and variant.
    pub token_index_pair: P::ResolvedPair<M>,

    /// Lot sizes (one per side)
    pub lot_size_pair: LotSizePair,

    /// Tick size (quote lots per base unit per tick)
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}
