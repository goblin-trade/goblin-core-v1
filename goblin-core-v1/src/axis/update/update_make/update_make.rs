use crate::{
    axis::{
        market::{market_spec::MarketSpec, Readables},
        occupancy::occupancy_marker::OccupancyMarker,
        update::{update_sign::UpdateSign, Decrease, Increase, UpdateEnum},
        AxisMarker,
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, Position},
    state::{resting_order::preimage::RestingOrderPreimage, Preimage},
};

pub struct UpdateResult {
    pub delta_base_lots: BaseLots,
    pub resting_order_closed: bool,
}

pub trait UpdateMake: UpdateSign + AxisMarker<Enum = UpdateEnum> {
    fn update_resting_order<'a, MS, OM>(
        base_lots: BaseLots,
        position: Position,
        readables: &Readables<MS>,
    ) -> Result<UpdateResult, GoblinError>
    where
        MS: MarketSpec,
        OM: OccupancyMarker,
    {
        let key = &mut RestingOrderPreimage {
            market_key: readables.market_readables.market_key,
            position,
        }
        .hash();

        // Direction reversed because of convention.
        // UM refers to increase or decrease in store balance. To increase store balance
        // decrease the resting order and vice versa.
        let (updated_resting_order, delta_base_lots) = match Self::VARIANT {
            UpdateEnum::Increase => {
                OM::decrease_resting_order(readables.msg_sender, base_lots, key)
            }
            UpdateEnum::Decrease => {
                OM::increase_resting_order(readables.msg_sender, base_lots, key)
            }
        }?;

        let resting_order_closed = updated_resting_order.base_lots == BaseLots::default();

        if !resting_order_closed {
            key.store(&updated_resting_order);
        }

        Ok(UpdateResult {
            delta_base_lots,
            resting_order_closed,
        })
    }
}

impl UpdateMake for Increase {}
impl UpdateMake for Decrease {}
