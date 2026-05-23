use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease, Increase, UpdateEnum},
    },
    goblin_error::GoblinError,
    instructions::PosHeader,
    state::bitmap::alias::InnerBitmap,
};

pub(super) fn process_update_cases<M, B, Q>(
    readables: &Readables<M, B, Q>,
    pos_header: PosHeader,
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
        (LegEnum::Base, UpdateEnum::Increase) => Increase::process_update::<M, B, Q, Base>(
            readables,
            pos_header,
            writables,
            inner_bitmap_state,
        ),
        (LegEnum::Quote, UpdateEnum::Increase) => Increase::process_update::<M, B, Q, Quote>(
            readables,
            pos_header,
            writables,
            inner_bitmap_state,
        ),
        (LegEnum::Base, UpdateEnum::Decrease) => Decrease::process_update::<M, B, Q, Base>(
            readables,
            pos_header,
            writables,
            inner_bitmap_state,
        ),
        (LegEnum::Quote, UpdateEnum::Decrease) => Decrease::process_update::<M, B, Q, Quote>(
            readables,
            pos_header,
            writables,
            inner_bitmap_state,
        ),
    }
}
