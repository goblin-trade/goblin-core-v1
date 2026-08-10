use crate::{
    axis::{
        market::{market_spec::MarketSpec, Writables},
        occupancy::Occupied,
        update::{update_make::UpdateMake, UpdateEnum},
    },
    goblin_error::GoblinError,
    instructions::MakeReadables,
    match_axes,
    matching::region::make_region::MakeRegion,
    require,
    state::bitmap::alias::InnerBitmap,
};

pub fn ix_update<MS: MarketSpec>(
    make_readables: &MakeReadables<MS>,
    update_enum: UpdateEnum,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError> {
    let position = make_readables.pos_header.position;
    let region = MakeRegion::new(&writables.market_state.last_positions, position);

    // Cannot update in `Spread` region as it has no orders
    let MakeRegion::In(leg_in) = region else {
        return Err(GoblinError::NoRestingOrder);
    };

    require!(
        inner_bitmap_state.index_active(position.into()),
        GoblinError::NoRestingOrder
    );

    match_axes!(In = leg_in, UM = update_enum => {
        UM::process_make::<MS, In, Occupied>(
            make_readables,
            writables,
            inner_bitmap_state,
        )?;
    });

    Ok(())
}
