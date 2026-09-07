use crate::{
    axis::{LegMatcher, OccupancyEnum, OccupancyMarker, UpdateEnum, UpdateMarker},
    axis_helpers::MarketSpec,
    matching::MakeRegion,
    quantities::Position,
    Ctx,
};

// Update last position in market state if new position is opened beyond the last stored position
pub(crate) fn update_last_position<MS, OM, UM, In>(
    position: Position,
    region: MakeRegion,
    ctx: &mut Ctx<MS>,
) where
    MS: MarketSpec,
    OM: OccupancyMarker,
    UM: UpdateMarker,
    In: LegMatcher,
{
    if OM::VARIANT == OccupancyEnum::Vacant
        && UM::VARIANT == UpdateEnum::Decrease
        && matches!(region, MakeRegion::OnLastPrice(_) | MakeRegion::Spread)
    {
        let last_position = In::get_leg_mut(&mut ctx.writables.market_state.last_positions);
        *last_position = position;
    }
}
