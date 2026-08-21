use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        occupancy::{OccupancyEnum, OccupancyMarker},
    },
    axis_helpers::MarketSpec,
    matching::region::make_region::MakeRegion,
    quantities::{InnerPos, Position},
    state::bitmap::alias::{InnerBitmap, InnerBitmapUpdater},
    Ctx,
};

pub(crate) fn update_matrix<MS, OM, In>(
    resting_order_empty: bool,
    position: Position,
    region: MakeRegion,
    inner_bitmap_state: &mut InnerBitmap,
    ctx: &mut Ctx<MS>,
) where
    MS: MarketSpec,
    OM: OccupancyMarker,
    In: LegMatcher,
{
    let mut inner_bitmap_updater = InnerBitmapUpdater {
        bitmap: inner_bitmap_state,
        pos: InnerPos::from(position),
    };

    if resting_order_empty {
        inner_bitmap_updater.deactivate();
    } else if OM::VARIANT == OccupancyEnum::Vacant {
        inner_bitmap_updater.activate();

        // Update last position if opened beyond the last stored position
        if let MakeRegion::OnLastPrice(_) | MakeRegion::Spread = region {
            let last_position = In::get_leg_mut(&mut ctx.writables.market_state.last_positions);
            *last_position = position;
        }
    }
}
