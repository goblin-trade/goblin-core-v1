use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_spec::MarketSpec, Writables},
        occupancy::Occupied,
        update::{update_make::UpdateMake, Decrease, Increase, UpdateEnum},
    },
    goblin_error::GoblinError,
    instructions::MakeReadables,
    state::bitmap::alias::InnerBitmap,
};

pub(super) fn process_update_cases<MS: MarketSpec>(
    make_readables: &MakeReadables<MS>,
    update_enum: UpdateEnum,
    leg_in: LegEnum,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError> {
    // TODO use for_axes!
    match (leg_in, update_enum) {
        (LegEnum::Base, UpdateEnum::Increase) => Increase::process_make::<MS, Base, Occupied>(
            make_readables,
            writables,
            inner_bitmap_state,
        ),
        (LegEnum::Quote, UpdateEnum::Increase) => Increase::process_make::<MS, Quote, Occupied>(
            make_readables,
            writables,
            inner_bitmap_state,
        ),
        (LegEnum::Base, UpdateEnum::Decrease) => Decrease::process_make::<MS, Base, Occupied>(
            make_readables,
            writables,
            inner_bitmap_state,
        ),
        (LegEnum::Quote, UpdateEnum::Decrease) => Decrease::process_make::<MS, Quote, Occupied>(
            make_readables,
            writables,
            inner_bitmap_state,
        ),
    }
}
