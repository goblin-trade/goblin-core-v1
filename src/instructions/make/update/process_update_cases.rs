use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, CommonMarket, MarketAndKey},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease, Increase, UpdateEnum},
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, Position, INNER_POS_V2},
    settlement::local_delta::LocalSenderDelta,
    state::{bitmap_v2::BitmapV2, resting_order::preimage::RestingOrderPreimage, SlotKey},
};

pub fn process_update_cases<M, B, Q>(
    local_sender_delta: &mut LocalSenderDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    position_2: Position,
    inner_bitmap_state: &mut BitmapV2<INNER_POS_V2>,
    base_lots: BaseLots,
    update_variant: UpdateEnum,
    leg_in: LegEnum,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    match (leg_in, update_variant) {
        (LegEnum::Base, UpdateEnum::Increase) => Increase::process_update::<M, B, Q, Base>(
            local_sender_delta,
            market_and_key,
            position_2,
            base_lots,
            inner_bitmap_state,
        ),
        (LegEnum::Quote, UpdateEnum::Increase) => Increase::process_update::<M, B, Q, Quote>(
            local_sender_delta,
            market_and_key,
            position_2,
            base_lots,
            inner_bitmap_state,
        ),
        (LegEnum::Base, UpdateEnum::Decrease) => Decrease::process_update::<M, B, Q, Base>(
            local_sender_delta,
            market_and_key,
            position_2,
            base_lots,
            inner_bitmap_state,
        ),

        (LegEnum::Quote, UpdateEnum::Decrease) => Decrease::process_update::<M, B, Q, Quote>(
            local_sender_delta,
            market_and_key,
            position_2,
            base_lots,
            inner_bitmap_state,
        ),
    }
}
