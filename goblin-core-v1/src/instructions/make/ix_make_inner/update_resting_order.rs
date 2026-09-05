use crate::{
    axis::{OccupancyMarker, UpdateMarker},
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    quantities::{BaseLots, Position},
    state::{Preimage, RestingOrderPreimage},
    Ctx,
};

pub(crate) fn update_resting_order<MS, OM, UM>(
    base_lots: BaseLots,
    position: Position,
    ctx: &mut Ctx<MS>,
) -> Result<(BaseLots, bool), GoblinError>
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

    let resting_order_empty = resting_order.base_lots == BaseLots::default();

    if !resting_order_empty {
        key.store(resting_order);
    }

    Ok((delta_base_lots, resting_order_empty))
}
