use crate::{
    goblin_error::GoblinError,
    market::MarketVariant,
    quantities::QuoteLotsPerBaseUnitPerTick,
    state::MarketPreimage,
    token::{CustomToken, TokenMarker},
    types::{Base, LegQuantities, Pair, Quote, Tuple, TupleReader},
};

pub type LotSizePair =
    Pair<<Base as LegQuantities>::LotsPerUnit, <Quote as LegQuantities>::LotsPerUnit>;

// If P needs to be a trait, we need to use T0 and T1 here
pub struct CommonMarket<M: MarketVariant, B: TokenMarker, Q: TokenMarker> {
    /// Lot sizes (one per side)
    pub lot_size_pair: LotSizePair,

    /// Tick size (quote lots per base unit per tick)
    pub tick_size: QuoteLotsPerBaseUnitPerTick,

    /// The token pair, parameterized by shape and variant.
    pub token_index_pair: Pair<B::TokenIndex<M>, Q::TokenIndex<M>>,
}

impl<M: MarketVariant, B: TokenMarker, Q: TokenMarker> CommonMarket<M, B, Q> {
    /// Map to market preimage which is used to read market state
    pub fn get_preimage(
        &self,
        custom_erc20_list: &[CustomToken],
    ) -> Result<MarketPreimage<M, B, Q>, GoblinError> {
        let base_token_index = Base::get(&self.token_index_pair);
        let quote_token_index = Quote::get(&self.token_index_pair);
        let base_token_address =
            B::token_index_to_address_outer(base_token_index, custom_erc20_list)?;
        let quote_token_address =
            Q::token_index_to_address_outer(quote_token_index, custom_erc20_list)?;

        Ok(MarketPreimage::<M, B, Q>::new(
            self.lot_size_pair,
            self.tick_size,
            Tuple::new(base_token_address, quote_token_address),
        ))
    }
}
