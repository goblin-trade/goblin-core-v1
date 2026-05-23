use crate::{
    axis::{
        market::{
            header::{
                inner_bitmap_header::InnerBitmapHeader, outer_bitmap_header::OuterBitmapHeader,
            },
            market_marker::MarketMarker,
            Readables, Writables,
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    quantities::{SafePosition, OUTER_POS, POS_0},
    settlement::local_delta::LocalDelta,
    state::{bitmap::Bitmap, MarketState},
};

impl OuterBitmapHeader {
    pub fn process<M, B, Q>(
        ctx: &DecodeCtx,
        writables: &mut Writables,
        readables: &Readables<M, B, Q>,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let Self {
            outer_bitmap_index,
            inner_bitmap_count,
        } = Self::try_decode(ctx)?;

        let pos_0 = SafePosition::<POS_0>::new(outer_bitmap_index);

        let (outer_bitmap_key, mut outer_bitmap_state) =
            Bitmap::<POS_0, OUTER_POS>::conditional_read(
                readables.market_and_key.market_key,
                &writables.market_state.last_positions,
                pos_0,
            );
        let outer_bitmap_clone = outer_bitmap_state;

        for _ in 0..inner_bitmap_count {
            InnerBitmapHeader::process(ctx, writables, &mut outer_bitmap_state, readables, pos_0)?;
        }

        outer_bitmap_state.conditional_write(&outer_bitmap_clone, &outer_bitmap_key);

        Ok(())
    }
}
