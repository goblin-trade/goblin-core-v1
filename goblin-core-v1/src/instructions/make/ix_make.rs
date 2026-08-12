use crate::{
    axis::{
        market::{header::make_header::MakeHeader, market_spec::MarketSpec, Readables, Writables},
        occupancy::occupancy_marker::OccupancyMarker,
    },
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    instructions::{MakeReadables, PosHeader},
    match_axes,
    quantities::{Pos2, SafePosition, POS_1},
    state::bitmap::alias::InnerBitmap,
};

pub fn ix_make<MS: MarketSpec>(
    ctx: &DecodeCtx,
    readables: &Readables<MS>,
    pos_1: SafePosition<POS_1>,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError> {
    let MakeHeader {
        inner_pos,
        occupancy_enum,
        inner_enum_raw,
        base_lots,
    } = MakeHeader::try_fixed_decode(ctx)?;

    let pos_2 = Pos2::new(pos_1, inner_pos);
    let position = pos_2.into();

    let make_readables = &MakeReadables {
        readables,
        pos_header: PosHeader {
            position,
            base_lots,
        },
    };

    match_axes!(OM = occupancy_enum => {
        OM::make(make_readables, inner_enum_raw, writables, inner_bitmap_state)?;
    });

    Ok(())

    // TODO use for_axes! and generic
    //
    // Axis- use occupancy axis
    // But how to deal with second variable axis?
    // Occupancy = Occupied, UpdateMarker
    // Occupancy = Vacant, LegMatcher
    // match make_variant {
    //     MakeVariant::Occupied(update_enum) => {
    //         ix_update(make_readables, update_enum, writables, inner_bitmap_state)
    //     }
    //     MakeVariant::Vacant(leg_enum) => {
    //         ix_open(make_readables, leg_enum, writables, inner_bitmap_state)
    //     }
    // }
}
