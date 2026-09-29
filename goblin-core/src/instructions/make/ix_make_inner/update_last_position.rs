use crate::{
    Ctx,
    axis::{
        leg::{LegCoordinates, leg_matcher::LegMatcher},
        occupancy::{OccupancyEnum, OccupancyMarker},
        update::{UpdateEnum, UpdateMarker},
    },
    axis_helpers::MarketSpec,
    matching::MakeRegion,
    quantities::{FullPos, POS_2},
    types::StoreReader,
};

// Update last position in market state if new position is opened beyond the last stored position
pub(crate) fn update_last_position<MS, OM, UM, In>(
    position: FullPos,
    region: MakeRegion,
    ctx: &mut Ctx<MS>,
) where
    MS: MarketSpec,
    OM: OccupancyMarker,
    UM: UpdateMarker,
    In: LegMatcher,
{
    if OM::VARIANT == OccupancyEnum::Vacant && UM::VARIANT == UpdateEnum::Decrease {
        let last_positions = &mut ctx.writables.market_state.last_positions;
        match region {
            // First order on a fresh market. Seat the opener on its own side and
            // push the opposite bound to the far edge of the price range
            MakeRegion::Unseeded => {
                *In::get_leg_mut(last_positions) = position;
                *In::Opposite::get_leg_mut(last_positions) = In::Opposite::end::<POS_2>();
            }
            MakeRegion::OnLastPrice(_) | MakeRegion::Spread => {
                *In::get_leg_mut(last_positions) = position;
            }
            MakeRegion::In(_) => {}
        }
    }
}
