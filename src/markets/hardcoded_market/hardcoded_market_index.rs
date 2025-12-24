use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
};

// TODO store B, Q as PhantomData?
pub struct HardcodedMarketIndex(pub usize);

impl Decodable<HardcodedMarketIndex> for HardcodedMarketIndex {
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<HardcodedMarketIndex, GoblinError> {
        let market_index_raw = args.decode::<u8>(offset, len)? as usize;
        Ok(HardcodedMarketIndex(market_index_raw))
    }
}
