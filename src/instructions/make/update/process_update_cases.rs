use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, Writables},
        occupancy::Occupied,
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease, Increase, UpdateEnum},
    },
    goblin_error::GoblinError,
    instructions::MakeReadables,
    state::bitmap::alias::InnerBitmap,
};

pub(super) fn process_update_cases<M, B, Q>(
    make_readables: &MakeReadables<M, B, Q>,
    update_enum: UpdateEnum,
    leg_in: LegEnum,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    match (leg_in, update_enum) {
        (LegEnum::Base, UpdateEnum::Increase) => {
            <Increase as UpdateMarker<Base>>::process_update::<M, B, Q, Occupied>(
                make_readables,
                writables,
                inner_bitmap_state,
            )
        }
        (LegEnum::Quote, UpdateEnum::Increase) => {
            <Increase as UpdateMarker<Quote>>::process_update::<M, B, Q, Occupied>(
                make_readables,
                writables,
                inner_bitmap_state,
            )
        }
        (LegEnum::Base, UpdateEnum::Decrease) => {
            <Decrease as UpdateMarker<Base>>::process_update::<M, B, Q, Occupied>(
                make_readables,
                writables,
                inner_bitmap_state,
            )
        }
        (LegEnum::Quote, UpdateEnum::Decrease) => {
            <Decrease as UpdateMarker<Quote>>::process_update::<M, B, Q, Occupied>(
                make_readables,
                writables,
                inner_bitmap_state,
            )
        }
    }
}
