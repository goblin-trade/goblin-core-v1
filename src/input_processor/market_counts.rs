use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsDecoder, Decodable},
    require,
    types::{MarketVariantPair, PairShapeTriple},
};

const BYTE_COUNT: usize = 3;

/// The number of markets of each type
pub type MarketCounts = MarketVariantPair<MarketPairShapeCounts, MarketPairShapeCounts>;

/// Market counts per pair shape
type MarketPairShapeCounts = PairShapeTriple<u8, u8, u8>;

impl Decodable<Self> for MarketCounts {
    fn decode(
        args: &super::ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<Self, crate::goblin_error::GoblinError> {
        require!(len >= *offset + BYTE_COUNT, GoblinError::InvalidPayload);

        let byte_0 = args.decode_unchecked::<u8>(*offset);
        let byte_1 = args.decode_unchecked::<u8>(*offset);
        let byte_2 = args.decode_unchecked::<u8>(*offset);

        let market_counts = MarketVariantPair::new(
            MarketPairShapeCounts::new(byte_0 & 0b0000_1111, byte_0 >> 4, byte_1 & 0b0000_1111),
            MarketPairShapeCounts::new(byte_1 >> 4, byte_2 & 0b0000_1111, byte_2 >> 4),
        );

        *offset += BYTE_COUNT;

        Ok(market_counts)
    }
}
