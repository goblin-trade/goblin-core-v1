use crate::{
    input_processor::Decodable,
    markets::{MarketVariantMap, PairShapeMap},
};

pub type PairShapeMarkets = PairShapeMap<u8>;
pub type MarketCounts = MarketVariantMap<PairShapeMarkets>;

impl Decodable<Self> for MarketCounts {
    fn decode(
        args: &super::ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<Self, crate::goblin_error::GoblinError> {
        todo!()
    }
}
