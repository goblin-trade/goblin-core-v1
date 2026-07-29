use crate::{
    axis::{
        leg::Pair,
        market::{token_pair::TokenPair, CommonMarket, Dynamic},
        token::token_quantity::TokenQuantity,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    require,
};

impl<TP: TokenPair> Decodable for CommonMarket<(Dynamic, TP)> {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        let base_token_index = <TP::Base as TokenQuantity>::TokenIndex::try_decode(ctx)?;
        let quote_token_index = <TP::Quote as TokenQuantity>::TokenIndex::try_decode(ctx)?;

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

        Ok(CommonMarket::<(Dynamic, TP)>::new(
            token_index_pair,
            lot_size_pair,
            tick_size,
        ))
    }
}
