use super::MarketHeader;
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
    instructions::ix_make,
    matching::region::make_region::MakeRegion,
    quantities::{
        InnerPosV2, OuterBitmapIndexV2, OuterPosV2, Position, INNER_POS_V2, OUTER_POS_V2,
    },
    settlement::local_delta::LocalDelta,
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        MarketState, Preimage,
    },
    types::Address,
};

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn execute_makes(
        &self,
        msg_sender: &Address,
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
            let position_0 = Position::from(outer_bitmap_index);

            let outer_bitmap_key = BitmapPreimageV2::<M, B, Q, OUTER_POS_V2> {
                market_key: market_and_key.market_key,
                position: position_0,
            }
            .hash();

            let region_0 = MakeRegion::new(&market_state.last_positions, position_0);

            let mut outer_bitmap_state = if region_0 == MakeRegion::Spread {
                BitmapV2::<OUTER_POS_V2>::default()
            } else {
                let outer_bitmap = outer_bitmap_key.load();
                if outer_bitmap.is_closed() {
                    BitmapV2::<OUTER_POS_V2>::default()
                } else {
                    outer_bitmap
                }
            };
            let outer_bitmap_clone = outer_bitmap_state;

            for _ in 0..outer_bitmap_header.inner_bitmap_count {
                let InnerBitmapHeader {
                    outer_pos,
                    update_count,
                } = InnerBitmapHeader::try_decode(ctx)?;

                let position_1 = position_0 + Position::from(outer_pos);
                let region_1 = MakeRegion::new(&market_state.last_positions, position_1);

                let inner_bitmap_key = BitmapPreimageV2::<M, B, Q, INNER_POS_V2> {
                    market_key: market_and_key.market_key,
                    position: position_1,
                }
                .hash();

                let mut inner_bitmap_state = if region_1 == MakeRegion::Spread
                    || !outer_bitmap_state.index_active(outer_pos)
                {
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
            }

            if outer_bitmap_clone != outer_bitmap_state {
                if outer_bitmap_state.is_empty() {
                    // Outer bitmap deactivated
                    outer_bitmap_state.close_with_sentinel();
                } else {
                    outer_bitmap_key.store(&outer_bitmap_state);
                }
            }
        }

        Ok(())
    }
}
