use core::marker::PhantomData;

use goblin_macros::FixedDecode;

use crate::{
    axis::{
        leg::{Base, Quote},
        token::token_reader::TokenDataTriple,
    },
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    market::{LotSizePair, TokenIndexPair, TokenPair},
    quantities::QuoteLotsPerBaseUnitPerTick,
    state::MarketPreimage,
    types::{LifetimedStoreReader, StoreReader, Tuple},
};

#[derive(FixedDecode)]
pub struct CommonMarket<MS: MarketSpec> {
    /// The token pair
    pub token_index_pair: TokenIndexPair<MS::Pair>,

    /// Lot sizes (one per side)
    pub lot_size_pair: LotSizePair,

    /// Tick size (quote lots per base unit per tick)
    pub tick_size: QuoteLotsPerBaseUnitPerTick,

    _marker: PhantomData<MS>,
}

impl<MS: MarketSpec> CommonMarket<MS> {
    pub const fn new(
        token_index_pair: TokenIndexPair<MS::Pair>,
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
    ) -> Result<MarketPreimage<MS>, GoblinError> {
        let base_token_index = Base::get(&self.token_index_pair);
        let quote_token_index = Quote::get(&self.token_index_pair);

        let base_data_list = <MS::Pair as TokenPair>::Base::get_lifetimed(token_data_triple);
        let quote_data_list = <MS::Pair as TokenPair>::Quote::get_lifetimed(token_data_triple);

        let base_token_address = base_data_list[base_token_index].address;
        let quote_token_address = quote_data_list[quote_token_index].address;

        Ok(MarketPreimage::<MS>::new(
            self.lot_size_pair,
            self.tick_size,
            Tuple::new(base_token_address, quote_token_address),
        ))
    }
}
