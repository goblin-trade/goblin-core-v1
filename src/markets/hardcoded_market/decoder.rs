use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    markets::{HardcodedMarket, HardcodedMarketList, PairShape},
};

/// `Decodable` impl for HardcodedMarket<P>
impl<P> Decodable<&'static HardcodedMarket<P>> for HardcodedMarket<P>
where
    P: PairShape + 'static,
    Self: HardcodedMarketList<P>,
{
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<&'static HardcodedMarket<P>, GoblinError> {
        let market_index_raw = args.decode::<u8>(offset, len)? as usize;

        Self::HARDCODED_MARKET_LIST
            .get(market_index_raw)
            .ok_or(GoblinError::InvalidHardcodedMarket)
    }
}
