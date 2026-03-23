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
    settlement::local_delta::LocalDelta,
    state::MarketState,
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

            for _ in 0..outer_bitmap_header.inner_bitmap_count {
                let inner_bitmap_header = InnerBitmapHeader::try_decode(ctx)?;

                for _ in 0..inner_bitmap_header.update_count {
                    ix_update::<M, B, Q, Base>(ctx, local_delta, market_and_key, market_state)?;
                }
            }
        }

        Ok(())
    }
}
