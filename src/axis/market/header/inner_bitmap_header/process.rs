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
    matching::region::make_region::MakeRegion,
    quantities::{Position, INNER_POS_V2, OUTER_POS_V2},
    settlement::local_delta::LocalDelta,
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        MarketState, Preimage,
    },
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
        let region_1 = MakeRegion::new(&market_state.last_positions, position_1);

        let inner_bitmap_key = BitmapPreimageV2::<M, B, Q, INNER_POS_V2> {
            market_key: market_and_key.market_key,
            position: position_1,
        }
        .hash();

        let mut inner_bitmap_state =
            if region_1 == MakeRegion::Spread || !outer_bitmap_state.index_active(outer_pos) {
                BitmapV2::<INNER_POS_V2>::default()
            } else {
                inner_bitmap_key.load()
            };

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

        if inner_bitmap_clone != inner_bitmap_state {
            if inner_bitmap_state.is_empty() {
                // Inner bitmap deactivated
                outer_bitmap_state.deactivate(outer_pos);
            } else if inner_bitmap_clone.is_empty() {
                // Inner Bitmap activated
                inner_bitmap_key.store(&inner_bitmap_state);
                outer_bitmap_state.activate(outer_pos);
            } else {
                // Inner bitmap updated
                inner_bitmap_key.store(&inner_bitmap_state);
            }
        }

        Ok(())
    }
}
