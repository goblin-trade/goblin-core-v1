use crate::{
    axis::{
        market::{
            header::inner_bitmap_header::InnerBitmapHeader, market_marker::MarketMarker, Readables,
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::MakeMutables,
    quantities::{SafePosition, INNER_POS, OUTER_POS, POS_0, POS_1},
    settlement::local_delta::LocalDelta,
    state::{bitmap::Bitmap, MarketState},
};

impl InnerBitmapHeader {
    pub fn process<M, B, Q>(
        ctx: &DecodeCtx,
        local_delta: &mut LocalDelta,
        market_state: &mut MarketState,
        outer_bitmap_state: &mut Bitmap<POS_0, OUTER_POS>,
        readables: &Readables<M, B, Q>,
        pos_0: SafePosition<POS_0>,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let Self {
            outer_pos,
            update_count,
        } = Self::try_decode(ctx)?;

        let pos_1 = SafePosition::<POS_1>::new(pos_0, outer_pos);

        let (inner_bitmap_key, mut inner_bitmap_state) =
            Bitmap::<POS_1, INNER_POS>::conditional_read(
                readables.market_and_key.market_key,
                &market_state.last_positions,
                pos_1,
                outer_bitmap_state,
                outer_pos,
            );
        let inner_bitmap_clone = inner_bitmap_state;

        let make_mutables = &mut MakeMutables {
            local_delta,
            market_state,
            inner_bitmap_state: &mut inner_bitmap_state,
        };

        for _ in 0..update_count {
            make_mutables.ix_make(ctx, readables, pos_1)?;
        }

        inner_bitmap_state.conditional_write(
            &inner_bitmap_clone,
            &inner_bitmap_key,
            outer_bitmap_state,
            outer_pos,
        );

        Ok(())
    }
}
