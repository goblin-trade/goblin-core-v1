use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    markets::{HardcodedMarket, HardcodedMarketList},
    token::TokenMarker,
};

impl<B, Q> Decodable<&'static HardcodedMarket<B, Q>> for HardcodedMarket<B, Q>
where
    B: TokenMarker + 'static,
    Q: TokenMarker + 'static,
    Self: HardcodedMarketList<B, Q>,
{
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<&'static HardcodedMarket<B, Q>, GoblinError> {
        let market_index_raw = args.decode::<u8>(offset, len)? as usize;

        Self::HARDCODED_MARKET_LIST
            .get(market_index_raw)
            .ok_or(GoblinError::InvalidHardcodedMarket)
    }
}
