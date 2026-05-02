use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::open::process_open::process_open,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, InnerPosV2, Position, INNER_POS_V2},
    require,
    settlement::local_delta::LocalDelta,
    state::{bitmap_v2::BitmapV2, MarketState},
    types::{Address, StoreReader},
};

pub fn ix_open<M, B, Q>(
    msg_sender: &Address,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    position_2: Position,
    region_2: MakeRegion,
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

    match leg_enum {
        LegEnum::Base => process_open::<M, B, Q, Base>(
            msg_sender,
            &mut local_delta.local_sender_delta,
            market_and_key,
            position_2,
            base_lots,
        ),
        LegEnum::Quote => process_open::<M, B, Q, Quote>(
            msg_sender,
            &mut local_delta.local_sender_delta,
            market_and_key,
            position_2,
            base_lots,
        ),
    }
}
