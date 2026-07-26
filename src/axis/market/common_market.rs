use core::marker::PhantomData;

use crate::{
    axis::{
        leg::{Base, Pair, Quote},
        market::{market_marker::MarketMarker, LotSizePair},
        token::{
            token_marker::TokenMarker, token_quantity::TokenQuantity, token_reader::TokenDataTriple,
        },
    },
    goblin_error::GoblinError,
    quantities::QuoteLotsPerBaseUnitPerTick,
    state::MarketPreimage,
    types::{StoreReader, Tuple},
};

pub type TokenIndexPair<B, Q> =
    Pair<<B as TokenQuantity>::TokenIndex, <Q as TokenQuantity>::TokenIndex>;

pub struct CommonMarket<M: MarketMarker, B: TokenMarker, Q: TokenMarker> {
    /// The token pair
    pub token_index_pair: TokenIndexPair<B, Q>,

    /// Lot sizes (one per side)
    pub lot_size_pair: LotSizePair,

    /// Tick size (quote lots per base unit per tick)
    pub tick_size: QuoteLotsPerBaseUnitPerTick,

    _marker: PhantomData<M>,
}

impl<M: MarketMarker, B: TokenMarker, Q: TokenMarker> CommonMarket<M, B, Q> {
    pub const fn new(
        token_index_pair: Pair<B::TokenIndex, Q::TokenIndex>,
        lot_size_pair: LotSizePair,
        tick_size: QuoteLotsPerBaseUnitPerTick,
    ) -> Self {
        Self {
            lot_size_pair,
            tick_size,
            token_index_pair,
            _marker: PhantomData,
        }
    }

    /// Map to market preimage which is used to read market state
    pub fn get_preimage<'a>(
        &self,
        token_data_triple: &TokenDataTriple<'a>,
    ) -> Result<MarketPreimage<M, B, Q>, GoblinError> {
        let base_token_index = Base::get(&self.token_index_pair);
        let quote_token_index = Quote::get(&self.token_index_pair);

        let base_data_list = B::get_lifetimed(token_data_triple);
        let quote_data_list = Q::get_lifetimed(token_data_triple);

        let base_token_address = base_data_list[base_token_index].address;
        let quote_token_address = quote_data_list[quote_token_index].address;

        Ok(MarketPreimage::<M, B, Q>::new(
            self.lot_size_pair,
            self.tick_size,
            Tuple::new(base_token_address, quote_token_address),
        ))
    }
}
