use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable, MarketCounts},
    require,
};

const BYTE_COUNT: usize = 3;

impl Decodable for MarketCounts {
    fn decode(args: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<Self, GoblinError> {
        require!(len >= *offset + BYTE_COUNT, GoblinError::InvalidPayload);

        let byte_0 = args.decode_unchecked::<u8>(*offset);
        let byte_1 = args.decode_unchecked::<u8>(*offset);
        let byte_2 = args.decode_unchecked::<u8>(*offset);
        *offset += BYTE_COUNT;

        Ok(Self::new([
            byte_0 & 0b0000_1111,
            byte_0 >> 4,
            byte_1 & 0b0000_1111,
            byte_1 >> 4,
            byte_2 & 0b0000_1111,
            byte_2 >> 4,
        ]))
    }
}
