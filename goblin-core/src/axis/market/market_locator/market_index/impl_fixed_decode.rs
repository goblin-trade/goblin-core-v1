use crate::{
    axis::market::{HardcodedMarketList, MarketIndex},
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    require,
};

impl<'a, TP> FixedDecode<'a> for MarketIndex<TP>
where
    TP: TokenPair + HardcodedMarketList,
{
    const ENCODED_SIZE: usize = 1;

    fn raw_fixed_decode(reader: &'a ArgsReader) -> Self {
        let market_index_raw = u8::raw_fixed_decode(reader) as usize;
        Self::new(market_index_raw)
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(
            self.inner < TP::HARDCODED_MARKET_LIST.len(),
            GoblinError::InvalidPayload
        );
        Ok(())
    }
}
