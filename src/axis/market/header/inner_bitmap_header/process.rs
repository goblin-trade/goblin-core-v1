use crate::{
    axis::{
        market::{
            header::inner_bitmap_header::InnerBitmapHeader, market_marker::MarketMarker,
            MarketAndKey,
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::ix_make,
    quantities::{Position, SafePosition, INNER_POS, OUTER_POS, POS_0, POS_1},
    settlement::local_delta::LocalDelta,
    state::{bitmap::Bitmap, MarketState},
    types::Address,
};

impl InnerBitmapHeader {
    pub fn process<M, B, Q>(
        msg_sender: &Address,
        ctx: &DecodeCtx,
        local_delta: &mut LocalDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        market_state: &mut MarketState,
        pos_0: SafePosition<POS_0>,
        outer_bitmap_state: &mut Bitmap<OUTER_POS>,
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

        let (inner_bitmap_key, mut inner_bitmap_state) = Bitmap::<INNER_POS>::conditional_read(
            market_and_key.market_key,
            &market_state.last_positions,
            pos_1,
            outer_bitmap_state,
            outer_pos,
        );
        let inner_bitmap_clone = inner_bitmap_state;

        for _ in 0..update_count {
            ix_make::<M, B, Q>(
                msg_sender,
                ctx,
                local_delta,
                market_and_key,
                market_state,
                pos_1,
                &mut inner_bitmap_state,
            )?;
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
