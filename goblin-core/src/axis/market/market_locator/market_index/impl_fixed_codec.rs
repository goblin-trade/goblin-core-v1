use super::{HardcodedMarketList, MarketIndex};
use crate::{
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, ArgsWriter, FixedCodec},
    require,
};

impl<TP> FixedCodec for MarketIndex<TP>
where
    TP: TokenPair + HardcodedMarketList,
{
    const ENCODED_SIZE: usize = 1;

    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
        let market_index_raw = u8::raw_fixed_decode(reader) as usize;
        Self::new(market_index_raw)
    }

    fn raw_fixed_encode(&self, writer: &mut ArgsWriter) {
        (self.inner as u8).raw_fixed_encode(writer);
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(
            self.inner < TP::HARDCODED_MARKET_LIST.len(),
            GoblinError::InvalidPayload
        );
        Ok(())
    }
}
