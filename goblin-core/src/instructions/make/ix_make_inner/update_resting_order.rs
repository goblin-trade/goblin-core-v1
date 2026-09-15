use crate::{
    Ctx,
    axis::{occupancy::OccupancyMarker, update::UpdateMarker},
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    quantities::{BaseLots, PositionV2},
    state::{Preimage, RestingOrderPreimage},
};

pub(crate) fn update_resting_order<MS, OM, UM>(
    base_lots: BaseLots<u64>,
    position: PositionV2,
    ctx: &mut Ctx<MS>,
) -> Result<(BaseLots<u64>, bool), GoblinError>
where
    MS: MarketSpec,
    OM: OccupancyMarker,
    UM: UpdateMarker,
{
    let key = &RestingOrderPreimage {
        market_key: ctx.readables.market_readables().market_key,
        position,
    }
    .hash();

    let resting_order = &mut OM::get_validated_resting_order(key, ctx.readables.msg_sender)?;
    let delta_base_lots = UM::update_resting_order(base_lots, resting_order)?;

    let resting_order_empty = resting_order.base_lots == BaseLots::<u64>::default();

    if !resting_order_empty {
        key.store(resting_order);
    }

    Ok((delta_base_lots, resting_order_empty))
}
