use crate::{
    markets::MarketVariant,
    quantities::QuoteLotsPerBaseUnitPerTick,
    token::TokenMarker,
    types::{Base, LegMarker, Pair, Quote, TokenIndexPair},
};

pub type LotSizePair = Pair<<Base as LegMarker>::LotsPerUnit, <Quote as LegMarker>::LotsPerUnit>;

// If P needs to be a trait, we need to use T0 and T1 here
pub struct CommonMarket<M: MarketVariant, B: TokenMarker, Q: TokenMarker> {
    /// The token pair, parameterized by shape and variant.
    pub token_index_pair: TokenIndexPair<M, B, Q>,

    /// Lot sizes (one per side)
    pub lot_size_pair: LotSizePair,

    /// Tick size (quote lots per base unit per tick)
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}
