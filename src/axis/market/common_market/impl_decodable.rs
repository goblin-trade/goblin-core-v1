use crate::{
    axis::market::{market_spec::MarketSpec, CommonMarket, LotSizePair, TokenIndexPair},
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx},
    quantities::QuoteLotsPerBaseUnitPerTick,
    require,
};

/// Decode CommonMarket
///
/// While this is generically implemented on M, we only decode dynamic markets and not
/// hardcoded markets.
impl<MS: MarketSpec> Decodable for CommonMarket<MS> {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        // Checked decode for token index pair as generics have diferent sizes.
        // ETH has 0 size.
        let token_index_pair = TokenIndexPair::<MS::Pair>::try_decode(ctx)?;

        // Batch decode and advance bounds
        require!(
            ctx.len() >= ctx.offset.get() + 3,
            GoblinError::InvalidPayload
        );
        let lot_size_pair = LotSizePair::decode_unchecked_no_advance(ctx);
        let tick_size = QuoteLotsPerBaseUnitPerTick::decode_unchecked_no_advance(ctx);

        ctx.advance_offset(3);

        Ok(CommonMarket::new(
            token_index_pair,
            lot_size_pair,
            tick_size,
        ))
    }
}
