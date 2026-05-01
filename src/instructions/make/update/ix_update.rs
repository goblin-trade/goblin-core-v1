use crate::{
    axis::{
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    instructions::update::process_update_cases,
    quantities::{BaseLots, Position, INNER_POS_V2},
    require,
    settlement::local_delta::LocalDelta,
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        resting_order::preimage::RestingOrderPreimage,
        MarketState, Preimage, SlotKey,
    },
};

pub fn ix_update<M, B, Q>(
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    position_2: Position,
    inner_bitmap_key: &SlotKey<BitmapPreimageV2<M, B, Q, INNER_POS_V2>>,
    inner_bitmap_state: &mut BitmapV2<INNER_POS_V2>,
    base_lots: BaseLots,
    update_enum: UpdateEnum,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    Ok(())
    // require!(
    //     inner_bitmap_state.pos_active(full_coordinates.inner_pos),
    //     GoblinError::NoRestingOrder
    // );

    // let resting_order_key = RestingOrderPreimage {
    //     inner_bitmap_key: *inner_bitmap_key,
    //     inner_pos: full_coordinates.inner_pos,
    // }
    // .hash();

    // process_update_cases::<M, B, Q>(
    //     &mut local_delta.local_sender_delta,
    //     &market_and_key.market,
    //     market_state,
    //     &resting_order_key,
    //     full_coordinates,
    //     inner_bitmap_state,
    //     base_lots,
    //     update_enum,
    // )
}
