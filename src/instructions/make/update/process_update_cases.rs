use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, Writables},
        occupancy::Occupied,
        token::token_marker::TokenMarker,
        update::{update_make::UpdateMake, Decrease, Increase, UpdateEnum},
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
        (LegEnum::Base, UpdateEnum::Increase) => Increase::process_make::<M, B, Q, Base, Occupied>(
            make_readables,
            writables,
            inner_bitmap_state,
        ),
        (LegEnum::Quote, UpdateEnum::Increase) => {
            Increase::process_make::<M, B, Q, Quote, Occupied>(
                make_readables,
                writables,
                inner_bitmap_state,
            )
        }
        (LegEnum::Base, UpdateEnum::Decrease) => Decrease::process_make::<M, B, Q, Base, Occupied>(
            make_readables,
            writables,
            inner_bitmap_state,
        ),
        (LegEnum::Quote, UpdateEnum::Decrease) => {
            Decrease::process_make::<M, B, Q, Quote, Occupied>(
                make_readables,
                writables,
                inner_bitmap_state,
            )
        }
    }
}
