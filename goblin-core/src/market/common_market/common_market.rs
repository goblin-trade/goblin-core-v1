use goblin_macros::FixedDecode;

use crate::{
    axis::{
        leg::{Base, Pair, Quote, SamePair},
        token::token_reader::TokenDataTriple,
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    market::{LotSizePair, TokenIndexPair},
    quantities::{ATOMS_PER_UNIT, QuoteLotsPerBaseUnitPerTick, UnsideQuantity, UnsidedAtomsPerLot},
    state::MarketPreimage,
};

#[derive(FixedDecode)]
pub struct CommonMarket<TP: TokenPair> {
    /// The token pair
    pub token_index_pair: TokenIndexPair<TP>,

    /// Lot sizes (one per side)
    pub lot_size_pair: LotSizePair,

    /// Tick size (quote lots per base unit per tick)
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

impl<TP: TokenPair> CommonMarket<TP> {
    pub const fn new(
        token_index_pair: TokenIndexPair<TP>,
        lot_size_pair: LotSizePair,
        tick_size: QuoteLotsPerBaseUnitPerTick,
    ) -> Self {
        Self {
            lot_size_pair,
            tick_size,
            token_index_pair,
        }
    }

    /// Map to market preimage which is used to read market state
    pub fn get_preimage<'a>(
        &self,
        token_data_triple: &TokenDataTriple<'a>,
    ) -> Result<MarketPreimage<TP>, GoblinError> {
        let base_data = token_data_triple.get_data::<(TP, Base)>(&self.token_index_pair);
        let quote_data = token_data_triple.get_data::<(TP, Quote)>(&self.token_index_pair);

        let address_pair = Pair::new(base_data.address, quote_data.address);

        Ok(MarketPreimage::new(
            self.lot_size_pair,
            self.tick_size,
            address_pair,
        ))
    }

    pub fn atoms_per_lot_pair(&self) -> SamePair<UnsidedAtomsPerLot> {
        ATOMS_PER_UNIT / self.lot_size_pair.unsided()
    }
}
