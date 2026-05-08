use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease, Increase, UpdateEnum},
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, Position, INNER_POS},
    settlement::local_delta::LocalSenderDelta,
    state::bitmap::Bitmap,
    types::Address,
};

pub fn process_update_cases<M, B, Q>(
    msg_sender: &Address,
    local_sender_delta: &mut LocalSenderDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    position_2: Position,
    inner_bitmap_state: &mut Bitmap<INNER_POS>,
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
            msg_sender,
            local_sender_delta,
            market_and_key,
            position_2,
            base_lots,
            inner_bitmap_state,
        ),
        (LegEnum::Quote, UpdateEnum::Increase) => Increase::process_update::<M, B, Q, Quote>(
            msg_sender,
            local_sender_delta,
            market_and_key,
            position_2,
            base_lots,
            inner_bitmap_state,
        ),
        (LegEnum::Base, UpdateEnum::Decrease) => Decrease::process_update::<M, B, Q, Base>(
            msg_sender,
            local_sender_delta,
            market_and_key,
            position_2,
            base_lots,
            inner_bitmap_state,
        ),

        (LegEnum::Quote, UpdateEnum::Decrease) => Decrease::process_update::<M, B, Q, Quote>(
            msg_sender,
            local_sender_delta,
            market_and_key,
            position_2,
            base_lots,
            inner_bitmap_state,
        ),
    }
}
