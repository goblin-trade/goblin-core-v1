use core::marker::PhantomData;

use crate::{
    goblin_error::GoblinError,
    market::MarketMarker,
    quantities::QuoteLotsPerBaseUnitPerTick,
    state::MarketPreimage,
    token::{CustomERC20Data, TokenMarker},
    types::{Base, LegQuantities, Pair, Quote, StoreReader, Tuple},
};

pub type LotSizePair =
    Pair<<Base as LegQuantities>::LotsPerUnit, <Quote as LegQuantities>::LotsPerUnit>;

pub struct CommonMarket<M: MarketMarker, B: TokenMarker, Q: TokenMarker> {
    /// The token pair
    pub token_index_pair: Pair<B::TokenIndex, Q::TokenIndex>,

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
    pub fn get_preimage(
        &self,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<MarketPreimage<M, B, Q>, GoblinError> {
        let base_token_index = Base::get(&self.token_index_pair);
        let quote_token_index = Quote::get(&self.token_index_pair);
        let base_token_address = B::token_index_to_address(base_token_index, custom_erc20_list)?;
        let quote_token_address = Q::token_index_to_address(quote_token_index, custom_erc20_list)?;

        Ok(MarketPreimage::<M, B, Q>::new(
            self.lot_size_pair,
            self.tick_size,
            Tuple::new(base_token_address, quote_token_address),
        ))
    }
}
