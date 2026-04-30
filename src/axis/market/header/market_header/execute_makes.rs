use super::MarketHeader;
use crate::{
    axis::{
        leg::SamePair,
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
    instructions::ix_make,
    quantities::{InnerPosV2, OuterBitmapIndexV2, OuterPosV2, Position, OUTER_POS_V2},
    require,
    settlement::local_delta::LocalDelta,
    state::{bitmap_v2::preimage::BitmapPreimageV2, MarketState, Preimage},
};

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn execute_updates(
        &self,
        ctx: &DecodeCtx,
        local_delta: &mut LocalDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        market_state: &mut MarketState,
    ) -> Result<(), GoblinError> {
        let outer_bitmap_index_pair = market_state
            .last_positions
            .into_pair::<OuterBitmapIndexV2>();

        let outer_pos_pair = market_state.last_positions.into_pair::<OuterPosV2>();
        let inner_pos_pair = market_state.last_positions.into_pair::<InnerPosV2>();

        for _ in 0..self.outer_bitmap_count {
            let outer_bitmap_header = OuterBitmapHeader::try_decode(ctx)?;

            let outer_bitmap_index = outer_bitmap_header.outer_bitmap_index;
            let position = Position::from(outer_bitmap_index);

            let outer_bitmap_key = BitmapPreimageV2::<M, B, Q, OUTER_POS_V2> {
                market_key: market_and_key.market_key,
                position,
            }
            .hash();

            // TODO revise cleaning and garbage logic
            let mut active_outer_bitmap = ActiveOuterBitmap::new_cleaned(
                &outer_bitmap_key,
                outer_bitmap_index,
                &outer_bitmap_index_pair,
                &outer_pos_pair,
            );

            for _ in 0..outer_bitmap_header.inner_bitmap_count {
                let InnerBitmapHeader {
                    outer_pos,
                    update_count,
                } = InnerBitmapHeader::try_decode(ctx)?;

                let inner_bitmap_key = InnerBitmapPreimage {
                    outer_bitmap_key,
                    outer_pos,
                }
                .hash();

                let mut inner_bitmap_state = if !active_outer_bitmap.pos_active(outer_pos) {
                    InnerBitmap::default()
                } else {
                    inner_bitmap_key.load()
                };

                for _ in 0..update_count {
                    ix_make::<M, B, Q>(
                        ctx,
                        local_delta,
                        market_and_key,
                        market_state,
                        outer_bitmap_header.outer_bitmap_index,
                        outer_pos,
                        &inner_bitmap_key,
                        &mut inner_bitmap_state,
                    )?;
                }

                if inner_bitmap_state.bitmap_inactive() {
                    active_outer_bitmap.deactivate(outer_pos);
                } else {
                    // TODO compare with original field or use a flag
                    // to decide whether to write or not? We don't want to
                    // write to bitmap if orders were not opened or closed
                    inner_bitmap_key.store(&inner_bitmap_state);
                }
            }

            if active_outer_bitmap.bitmap_inactive() {}
        }

        Ok(())
    }
}
