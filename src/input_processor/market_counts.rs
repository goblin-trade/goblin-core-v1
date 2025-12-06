use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsDecoder, Decodable},
    markets::{MarketVariantMap, PairShapeMap},
    require,
};

const BYTE_COUNT: usize = 3;

/// The number of markets of each type
pub type MarketCounts = MarketVariantMap<PairShapeMarkets>;

/// Market counts per pair shape
pub type PairShapeMarkets = PairShapeMap<u8>;

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

        let market_counts = MarketVariantMap {
            hardcoded: PairShapeMap {
                eth_erc20: byte_0 & 0b0000_1111,
                erc20_eth: byte_0 >> 4,
                erc20_erc20: byte_1 & 0b0000_1111,
            },
            dynamic: PairShapeMap {
                eth_erc20: byte_1 >> 4,
                erc20_eth: byte_2 & 0b0000_1111,
                erc20_erc20: byte_2 >> 4,
            },
        };

        *offset += BYTE_COUNT;

        Ok(market_counts)
    }
}
