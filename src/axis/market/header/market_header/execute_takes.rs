use super::MarketHeader;
use crate::{
    axis::{
        leg::{Base, Quote},
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
    instructions::{ix_take, ix_update::ix_update},
    require,
    settlement::local_delta::LocalDelta,
    state::{
        bitmap::{
            inner_bitmap::preimage::InnerBitmapPreimage,
            outer_bitmap::{
                active_outer_bitmap, outer_bitmap_state::OuterBitmapState,
                preimage::OuterBitmapPreimage,
            },
            Bitmap,
        },
        MarketState, Preimage,
    },
    types::StoreReader,
};

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub fn execute_takes(
        &self,
        ctx: &DecodeCtx,
        local_delta: &mut LocalDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        market_state: &mut MarketState,
    ) -> Result<(), GoblinError> {
        if Base::get(&self.execute_takes) {
            ix_take::<M, B, Q, Base>(ctx, local_delta, market_and_key, market_state)?;
        }
        if Quote::get(&self.execute_takes) {
            ix_take::<M, B, Q, Quote>(ctx, local_delta, market_and_key, market_state)?;
        }

        Ok(())
    }

    pub fn execute_updates(
        &self,
        ctx: &DecodeCtx,
        local_delta: &mut LocalDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        market_state: &mut MarketState,
    ) -> Result<(), GoblinError> {
        for _ in 0..self.outer_bitmap_count {
            let outer_bitmap_header = OuterBitmapHeader::try_decode(ctx)?;

            let outer_bitmap_key = OuterBitmapPreimage {
                market_key: market_and_key.market_key,
                outer_bitmap_index: outer_bitmap_header.outer_bitmap_index,
            }
            .hash();

            let outer_bitmap_state = OuterBitmapState::from(outer_bitmap_key.load());

            let OuterBitmapState::Active(mut active_outer_bitmap) = outer_bitmap_state else {
                return Err(GoblinError::NoRestingOrder);
            };

            for _ in 0..outer_bitmap_header.inner_bitmap_count {
                let InnerBitmapHeader {
                    outer_pos,
                    update_count,
                } = InnerBitmapHeader::try_decode(ctx)?;

                require!(
                    active_outer_bitmap.pos_active(outer_pos),
                    GoblinError::NoRestingOrder
                );

                let inner_bitmap_key = InnerBitmapPreimage {
                    outer_bitmap_key,
                    outer_pos,
                }
                .hash();
                let mut inner_bitmap_state = inner_bitmap_key.load();

                for _ in 0..update_count {
                    ix_update::<M, B, Q>(
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
