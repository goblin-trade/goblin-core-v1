use crate::{
    axis::{
        market::market_marker::MarketMarker,
        occupancy::{occupancy_marker::OccupancyMarker, Vacant},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    quantities::BaseLots,
    state::{
        bitmap::alias::InnerBitmapUpdater,
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
    types::Address,
};

impl OccupancyMarker for Vacant {
    fn increase<'a, M, B, Q>(
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
        inner_bitmap_updater.activate();
        key.store(&RestingOrder {
            maker: *msg_sender,
            base_lots,
        });

        Ok(base_lots)
    }

    fn decrease<'a, M, B, Q>(
        _msg_sender: &Address,
        _base_lots: BaseLots,
        _key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        _inner_bitmap_updater: &mut InnerBitmapUpdater<'a>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        unreachable!()
    }
}
