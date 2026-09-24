use crate::codec::{CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};

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

pub struct CommonMarket<TP: TokenPair> {
    /// The token pair
    pub token_index_pair: TokenIndexPair<TP>,

    /// Lot sizes (one per side)
    pub lot_size_pair_u32: LotSizePair<u32>,

    /// Tick size (quote lots per base unit per tick)
    pub tick_size_u32: QuoteLotsPerBaseUnitPerTick<u32>,
}

impl<'de, TP: TokenPair> GoblinRead<'de, ()> for CommonMarket<TP> {
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let token_index_pair = TokenIndexPair::<TP>::from_reader_with_ctx(reader, ())?;
        let lot_size_pair_u32 = LotSizePair::<u32>::from_reader_with_ctx(reader, ())?;
        let tick_size_u32 = QuoteLotsPerBaseUnitPerTick::<u32>::from_reader_with_ctx(reader, ())?;

        Ok(Self {
            token_index_pair,
            lot_size_pair_u32,
            tick_size_u32,
        })
    }
}

#[cfg(feature = "encode")]
impl<TP: TokenPair> GoblinWrite<()> for CommonMarket<TP> {
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        self.token_index_pair.to_writer(writer, ())?;
        self.lot_size_pair_u32.to_writer(writer, ())?;
        self.tick_size_u32.to_writer(writer, ())?;
        Ok(())
    }
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
