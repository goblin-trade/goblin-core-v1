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
    matching::region::make_region::MakeRegion,
    quantities::{Position, OUTER_POS_V2},
    settlement::local_delta::LocalDelta,
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        MarketState, Preimage,
    },
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
        let outer_bitmap_header = Self::try_decode(ctx)?;

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
