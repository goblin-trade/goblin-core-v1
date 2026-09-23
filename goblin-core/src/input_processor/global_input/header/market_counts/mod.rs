mod impl_deku_reader;
#[cfg(feature = "encode")]
mod impl_deku_writer;

use crate::{
    axis::{market::Market, token::Token},
    axis_helpers::{MarketSpec, TokenPair},
    types::{SameTriple, SameTuple, StoreReader},
};

pub type MarketCounts = SameTuple<MarketCountsInner, Market>;
pub type MarketCountsInner = SameTriple<SameTriple<u8, Token>, Token>;

impl MarketCounts {
    pub fn get_count<MS: MarketSpec>(&self) -> u8 {
        let market_counts = MS::Market::get_leg(self);
        let base_counts = <MS::Pair as TokenPair>::Base::get_leg(market_counts);

        <MS::Pair as TokenPair>::Quote::get(base_counts)
    }
}
