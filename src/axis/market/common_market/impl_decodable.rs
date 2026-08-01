use core::todo;

use crate::{
    axis::{
        leg::{leg_quantities::LegQuantities, Base, Pair},
        market::{market_spec::MarketSpec, CommonMarket, LotSizePair, TokenIndexPair, TokenPair},
        token::token_quantity::TokenQuantity,
    },
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
        // TODO improve decoding utilities
        // In most cases decoding is the same
        // Check bounds, decode, advance offset
        //
        // If implemented on basic types we should have a generic way to get decoded
        // structs
        //
        // Cases
        // - Simple decode. Construct complex struct from simple units.
        // - Advanced: interpret bits. This needs custom implementation.
        let size = core::mem::size_of::<Self>();

        require!(
            ctx.len() >= ctx.offset.get() + size,
            GoblinError::InvalidPayload
        );

        ctx.advance_offset(size);

        todo!()

        // Ok(CommonMarket::)

        // // Checked decode for token index pair as generics have diferent sizes.
        // // ETH has 0 size.
        // let token_index_pair = Pair::new(
        //     <<MS::Pair as TokenPair>::Base as TokenQuantity>::TokenIndex::decode_unchecked(ctx)?,
        //     <<MS::Pair as TokenPair>::Quote as TokenQuantity>::TokenIndex::try_decode(ctx)?,
        // );

        // let lot_size_pair = Pair::new(
        //     <Base as LegQuantities>::LotsPerUnit::decode_unchecked(ctx)?,
        //     <<MS::Pair as TokenPair>::Quote as TokenQuantity>::TokenIndex::try_decode(ctx)?,
        // );
        // let lot_size_pair = LotSizePair::decode_unchecked_no_advance(ctx);
        // let tick_size = QuoteLotsPerBaseUnitPerTick::decode_unchecked_no_advance(ctx);

        // ctx.advance_offset(3);

        // Ok(CommonMarket::new(
        //     token_index_pair,
        //     lot_size_pair,
        //     tick_size,
        // ))
    }
}
