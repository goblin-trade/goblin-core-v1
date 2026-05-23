use crate::{
    axis::{
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    instructions::{update::process_update_cases::process_update_cases, PosHeader},
    matching::region::make_region::MakeRegion,
    require,
    state::bitmap::alias::InnerBitmap,
};

pub fn ix_update<M, B, Q>(
    readables: &Readables<M, B, Q>,
    pos_header: PosHeader,
    update_enum: UpdateEnum,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    let position = pos_header.position;
    let region = MakeRegion::new(&writables.market_state.last_positions, position);

    // Cannot update in `Spread` region as it has no orders
    let MakeRegion::In(leg_in) = region else {
        return Err(GoblinError::NoRestingOrder);
    };

    require!(
        inner_bitmap_state.index_active(position.into()),
        GoblinError::NoRestingOrder
    );

    process_update_cases(
        readables,
        pos_header,
        update_enum,
        leg_in,
        writables,
        inner_bitmap_state,
    )
}
