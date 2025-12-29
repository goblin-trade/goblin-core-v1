use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{CommonMarket, Dynamic},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    require,
    token::TokenMarker,
    types::Pair,
};

impl<'a, B, Q> Decodable<'a> for CommonMarket<Dynamic, B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
    B::TokenIndex<Dynamic>: Decodable<'a>,
    Q::TokenIndex<Dynamic>: Decodable<'a>,
{
    fn decode(ctx: &DecodeCtx<'a>) -> Result<Self, GoblinError> {
        let base_token_index = B::TokenIndex::<Dynamic>::decode(ctx)?;
        let quote_token_index = Q::TokenIndex::<Dynamic>::decode(ctx)?;

        let token_index_pair = Pair::new(base_token_index, quote_token_index);

        require!(
            ctx.len() >= ctx.offset.get() + 3,
            GoblinError::InvalidPayload
        );

        let lot_size_pair = Pair::new(
            ctx.decode_unchecked_no_advance::<BaseLotsPerBaseUnit>(),
            ctx.decode_unchecked_no_advance::<QuoteLotsPerQuoteUnit>(),
        );
        let tick_size = ctx.decode_unchecked_no_advance::<QuoteLotsPerBaseUnitPerTick>();

        ctx.advance_offset(3);

        Ok(CommonMarket::<Dynamic, B, Q> {
            token_index_pair,
            lot_size_pair,
            tick_size,
        })
    }
}
