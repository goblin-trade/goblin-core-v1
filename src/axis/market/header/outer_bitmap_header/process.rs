use crate::{
    axis::{
        market::{
            header::{
                inner_bitmap_header::InnerBitmapHeader, outer_bitmap_header::OuterBitmapHeader,
            },
            market_marker::MarketMarker,
            MarketAndKey,
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    quantities::{Position, OUTER_POS_V2},
    settlement::local_delta::LocalDelta,
    state::{bitmap_v2::BitmapV2, MarketState},
    types::Address,
};

impl OuterBitmapHeader {
    pub fn process<M, B, Q>(
        msg_sender: &Address,
        ctx: &DecodeCtx,
        local_delta: &mut LocalDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        market_state: &mut MarketState,
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

        let position_0 = Position::from(outer_bitmap_index);

        let (outer_bitmap_key, mut outer_bitmap_state) = BitmapV2::<OUTER_POS_V2>::conditional_read(
            market_and_key.market_key,
            &market_state.last_positions,
            position_0,
        );
        let outer_bitmap_clone = outer_bitmap_state;

        for _ in 0..inner_bitmap_count {
            InnerBitmapHeader::process(
                msg_sender,
                ctx,
                local_delta,
                market_and_key,
                market_state,
                position_0,
                &mut outer_bitmap_state,
            )?;
        }

        outer_bitmap_state.conditional_write(&outer_bitmap_clone, &outer_bitmap_key);

        Ok(())
    }
}
