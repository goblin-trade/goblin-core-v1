use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    instructions::open::process_open_cases::process_open_cases,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, InnerPosV2, Position, INNER_POS_V2},
    require,
    settlement::local_delta::LocalDelta,
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        resting_order::preimage::RestingOrderPreimage,
        MarketState, Preimage, SlotKey,
    },
    types::StoreReader,
};

pub fn ix_open<M, B, Q>(
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    position_2: Position,
    region_2: MakeRegion,
    inner_bitmap_key: &SlotKey<BitmapPreimageV2<M, B, Q, INNER_POS_V2>>,
    inner_bitmap_state: &mut BitmapV2<INNER_POS_V2>,
    base_lots: BaseLots,
    leg_enum: LegEnum,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    let inner_pos = InnerPosV2::from(position_2);

    // Ensure that we open on the correct side.
    // leg_in must match or the order must be opened within the spread region.
    if let MakeRegion::In(leg_in) = region_2 {
        require!(leg_in == leg_enum, GoblinError::InvalidOpenPrice);
        require!(
            !inner_bitmap_state.index_active(inner_pos),
            GoblinError::PositionOccupied
        );
    } else {
        // update last price if order is placed in spread
        let last_position = match leg_enum {
            LegEnum::Base => Base::get_leg_mut(&mut market_state.last_positions),
            LegEnum::Quote => Quote::get_leg_mut(&mut market_state.last_positions),
        };
        *last_position = position_2;
    }

    inner_bitmap_state.activate(inner_pos);

    let resting_order_key = RestingOrderPreimage {
        market_key: market_and_key.market_key,
        position: position_2,
    }
    .hash();

    match leg_enum {
        LegEnum::Base => process_open_cases::<M, B, Q, Base>(
            local_delta,
            market_and_key,
            market_state,
            position_2,
            region_2,
            inner_bitmap_key,
            inner_bitmap_state,
            base_lots,
        ),
        LegEnum::Quote => process_open_cases::<M, B, Q, Quote>(
            local_delta,
            market_and_key,
            market_state,
            position_2,
            region_2,
            inner_bitmap_key,
            inner_bitmap_state,
            base_lots,
        ),
    }
}
