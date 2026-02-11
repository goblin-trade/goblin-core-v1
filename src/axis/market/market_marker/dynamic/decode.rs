use crate::{
    axis::{
        leg::Pair,
        market::{CommonMarket, Dynamic},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    require,
};

impl<B, Q> Decodable for CommonMarket<Dynamic, B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let base_token_index = B::TokenIndex::try_decode(ctx)?;
        let quote_token_index = Q::TokenIndex::try_decode(ctx)?;

        let token_index_pair = Pair::new(base_token_index, quote_token_index);

        require!(
            ctx.len() >= ctx.offset.get() + 3,
            GoblinError::InvalidPayload
        );

        let lot_size_pair = Pair::new(
            BaseLotsPerBaseUnit::decode_unchecked_no_advance(ctx),
            QuoteLotsPerQuoteUnit::decode_unchecked_no_advance(ctx),
        );
        let tick_size = QuoteLotsPerBaseUnitPerTick::decode_unchecked_no_advance(ctx);

        ctx.advance_offset(3);

        Ok(CommonMarket::<Dynamic, B, Q>::new(
            token_index_pair,
            lot_size_pair,
            tick_size,
        ))
    }
}
