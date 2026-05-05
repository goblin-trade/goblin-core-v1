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
    quantities::{Position, INNER_POS_V2, OUTER_POS_V2},
    settlement::local_delta::LocalDelta,
    state::{bitmap_v2::BitmapV2, MarketState},
    types::Address,
};

impl InnerBitmapHeader {
    pub fn process<M, B, Q>(
        msg_sender: &Address,
        ctx: &DecodeCtx,
        local_delta: &mut LocalDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        market_state: &mut MarketState,
        position_0: Position,
        outer_bitmap_state: &mut BitmapV2<OUTER_POS_V2>,
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

        let position_1 = position_0 + Position::from(outer_pos);

        let (inner_bitmap_key, mut inner_bitmap_state) = BitmapV2::<INNER_POS_V2>::conditional_read(
            market_and_key.market_key,
            &market_state.last_positions,
            position_1,
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
                position_1,
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
