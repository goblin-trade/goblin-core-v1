use crate::{
    axis::{
        leg::Base,
        market::{header::make_header::MakeHeader, market_spec::MarketSpec, Readables, Writables},
        occupancy::occupancy_marker::OccupancyMarker,
        update::update_make::UpdateMake,
    },
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    match_axes,
    matching::region::make_region::MakeRegion,
    quantities::{InnerPos, Pos2, SafePosition, Ticks, POS_1},
    state::{
        bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
        resting_order::preimage::RestingOrderPreimage,
        Preimage,
    },
    types::StoreReader,
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

    let region = MakeRegion::new(&writables.market_state.last_positions, position);

    match_axes!(OM = occupancy_enum => {
        let enums = OM::get_enums(inner_enum_raw, region)?;

        match_axes!(UM = enums.0, In = enums.1 => {
            OM::validate_and_update_region::<In>(region, position, &mut writables.market_state.last_positions, inner_bitmap_state)?;

            let key = &mut RestingOrderPreimage {
                market_key: readables.market_readables.market_key,
                position,
            }
            .hash();

            // This will map back on OM
            let updated_base_lots = UM::update_resting_order::<MS, In, OM>(
                readables.msg_sender,
                base_lots,
                key,
                &mut InnerBitmapUpdater {
                    bitmap: inner_bitmap_state,
                    pos: InnerPos::from(position),
                },
            )?;

            let base_lot_size = Base::get(&readables.market_readables.market.lot_size_pair);
            let tick_size = readables.market_readables.market.tick_size;
            let price = Ticks::from(position);

            writables.local_delta.make.add_make::<In, UM>(
                updated_base_lots,
                base_lot_size,
                tick_size,
                price,
            )?;
        });

    });

    Ok(())
}
