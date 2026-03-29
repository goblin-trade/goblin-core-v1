use crate::{
    axis::{
        market::{header::update_header::UpdateHeader, market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::process_update_cases::process_update_cases,
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, FullCoordinates,
    },
    require,
    settlement::local_delta::LocalDelta,
    state::{
        bitmap::{
            inner_bitmap::{preimage::InnerBitmapPreimage, InnerBitmap},
            Bitmap,
        },
        resting_order::preimage::RestingOrderPreimage,
        MarketState, Preimage, SlotKey,
    },
};

pub fn ix_update<M, B, Q>(
    ctx: &DecodeCtx,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    outer_bitmap_index: OuterBitmapIndex,
    outer_pos: OuterPos,
    inner_bitmap_key: &SlotKey<InnerBitmapPreimage<M, B, Q>>,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    let header = UpdateHeader::try_decode(ctx)?;
    require!(
        inner_bitmap_state.pos_active(header.inner_pos),
        GoblinError::NoRestingOrder
    );

    let full_coordinates = FullCoordinates {
        outer_bitmap_index,
        outer_pos,
        inner_pos: header.inner_pos,
    };
    let resting_order_key = RestingOrderPreimage {
        inner_bitmap_key: *inner_bitmap_key,
        inner_pos: full_coordinates.inner_pos,
    }
    .hash();

    process_update_cases::<M, B, Q>(
        &mut local_delta.local_sender_delta,
        &market_and_key.market,
        market_state,
        &resting_order_key,
        full_coordinates,
        &header,
        inner_bitmap_state,
    )
}
