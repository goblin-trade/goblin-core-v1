use crate::{
    axis::{
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    instructions::update::process_update_cases,
    matching::bitmap::FullCoordinates,
    quantities::BaseLots,
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
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    full_coordinates: &FullCoordinates,
    inner_bitmap_key: &SlotKey<InnerBitmapPreimage<M, B, Q>>,
    inner_bitmap_state: &mut InnerBitmap,
    base_lots: BaseLots,
    update_variant: UpdateEnum,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    require!(
        inner_bitmap_state.pos_active(full_coordinates.inner_pos),
        GoblinError::NoRestingOrder
    );

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
        inner_bitmap_state,
        base_lots,
        update_variant,
    )
}
