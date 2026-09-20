use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

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

#[derive(DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct CommonMarket<TP: TokenPair> {
    /// The token pair
    pub token_index_pair: TokenIndexPair<TP>,

    /// Lot sizes (one per side)
    pub lot_size_pair_u32: LotSizePair<u32>,

    /// Tick size (quote lots per base unit per tick)
    pub tick_size_u32: QuoteLotsPerBaseUnitPerTick<u32>,
}

impl<TP: TokenPair> CommonMarket<TP> {
    pub const fn new(
        token_index_pair: TokenIndexPair<TP>,
        lot_size_pair_u32: LotSizePair<u32>,
        tick_size_u32: QuoteLotsPerBaseUnitPerTick<u32>,
    ) -> Self {
        Self {
            lot_size_pair_u32,
            tick_size_u32,
            token_index_pair,
        }
    }

    /// Map to market preimage which is used to read market state
    pub const fn get_preimage<'a>(
        &self,
        token_data_triple: &TokenDataTriple<'a>,
    ) -> Result<MarketPreimage<TP>, GoblinError> {
        let base_data = token_data_triple.get_data::<(TP, Base)>(&self.token_index_pair);
        let quote_data = token_data_triple.get_data::<(TP, Quote)>(&self.token_index_pair);

        let token_address_pair = Pair::new(base_data.address, quote_data.address);

        Ok(MarketPreimage {
            lot_size_pair_u32: self.lot_size_pair_u32,
            tick_size_u32: self.tick_size_u32,
            token_address_pair,
        })
    }

    pub fn atoms_per_lot_pair(&self) -> SamePair<UnsidedAtomsPerLot<u64>> {
        let lot_size_pair = LotSizePair::<u64>::from(&self.lot_size_pair_u32);
        ATOMS_PER_UNIT / lot_size_pair.unsided()
    }
}
