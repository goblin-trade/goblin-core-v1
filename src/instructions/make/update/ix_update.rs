use crate::{
    axis::{
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    instructions::update::process_update_cases,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, Position, INNER_POS_V2},
    require,
    settlement::local_delta::LocalDelta,
    state::{
        bitmap_v2::BitmapV2, resting_order::preimage::RestingOrderPreimage, MarketState, Preimage,
    },
};

pub fn ix_update<M, B, Q>(
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    position_2: Position,
    region_2: MakeRegion,
    inner_bitmap_state: &mut BitmapV2<INNER_POS_V2>,
    base_lots: BaseLots,
    update_enum: UpdateEnum,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    let MakeRegion::In(leg_in) = region_2 else {
        return Err(GoblinError::NoRestingOrder);
    };

    require!(
        inner_bitmap_state.index_active(position_2.into()),
        GoblinError::NoRestingOrder
    );

    let resting_order_key = RestingOrderPreimage {
        market_key: market_and_key.market_key,
        position: position_2,
    }
    .hash();

    process_update_cases::<M, B, Q>(
        &mut local_delta.local_sender_delta,
        &market_and_key.market,
        market_state,
        &resting_order_key,
        position_2,
        inner_bitmap_state,
        base_lots,
        update_enum,
        leg_in,
    )
}
