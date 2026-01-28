use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{MarketHeader, MarketVariant},
    token::TokenMarker,
    types::Tuple,
};

impl<M, B, Q> Decodable for MarketHeader<M, B, Q>
where
    M: MarketVariant<B, Q>,
    B: TokenMarker,
    Q: TokenMarker,
{
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let byte_0 = u8::try_decode(ctx)?;

        let decode_deposit_amounts = (byte_0 & 0b0000_0001) != 0;

        let execute_takes = Tuple::new(
            // base
            (byte_0 & 0b0000_0010) != 0,
            // quote
            (byte_0 & 0b0000_0100) != 0,
        );

        // 2 bits- max value 3
        // Too less, decipher one more byte
        // This field is currently unused. Increase the amount if needed by reading a new byte.
        let outer_bitmap_indices = (byte_0 & 0b0001_1000) >> 3;

        Ok(Self::new(
            decode_deposit_amounts,
            execute_takes,
            outer_bitmap_indices,
        ))
    }
}
