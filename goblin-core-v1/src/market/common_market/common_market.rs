use core::marker::PhantomData;

use goblin_macros::FixedDecode;

use crate::{
    axis::{
        leg::{Base, Pair, Quote},
        token::token_reader::TokenDataTriple,
    },
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    market::{LotSizePair, TokenIndexPair},
    quantities::QuoteLotsPerBaseUnitPerTick,
    state::MarketPreimage,
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
        let base_data = token_data_triple.get_data::<(MS::Pair, Base)>(&self.token_index_pair);
        let quote_data = token_data_triple.get_data::<(MS::Pair, Quote)>(&self.token_index_pair);

        let address_pair = Pair::new(base_data.address, quote_data.address);

        Ok(MarketPreimage::<MS>::new(
            self.lot_size_pair,
            self.tick_size,
            address_pair,
        ))
    }
}
