use crate::{
    axis::{
        market::market_marker::MarketMarker,
        occupancy::{occupancy_marker::OccupancyMarker, Occupied},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    quantities::BaseLots,
    require,
    settlement::CheckedOps,
    state::{
        bitmap::alias::InnerBitmapUpdater, resting_order::preimage::RestingOrderPreimage, SlotKey,
    },
    types::Address,
};

impl OccupancyMarker for Occupied {
    fn increase_resting_order<'a, M, B, Q>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        _inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let mut resting_order = key.load();

        require!(
            resting_order.maker == *msg_sender,
            GoblinError::UnauthorizedMsgSender
        );

        let stored_base_lots = &mut resting_order.base_lots;
        *stored_base_lots = stored_base_lots
            .checked_add(base_lots)
            .ok_or(GoblinError::Overflow)?;

        key.store(&resting_order);

        Ok(base_lots)
    }

    fn decrease_resting_order<'a, M, B, Q>(
        msg_sender: &Address,
        base_lots: BaseLots,
        key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let mut resting_order = key.load();

        require!(
            resting_order.maker == *msg_sender,
            GoblinError::UnauthorizedMsgSender
        );

        let stored_base_lots = &mut resting_order.base_lots;
        let reduced_lots = if *stored_base_lots > base_lots {
            *stored_base_lots -= base_lots;
            key.store(&resting_order);

            base_lots
        } else {
            inner_bitmap_updater.deactivate();

            *stored_base_lots
        };

        Ok(reduced_lots)
    }
}
