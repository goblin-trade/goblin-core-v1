use crate::{
    axis::{
        market::market_marker::MarketMarker,
        occupancy::{occupancy_marker::OccupancyMarker, Vacant},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, InnerPos},
    state::{
        bitmap::alias::InnerBitmap,
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
    types::Address,
};

impl OccupancyMarker for Vacant {
    fn increase<M, B, Q>(
        msg_sender: &Address,
        base_lots: BaseLots,
        _inner_pos: InnerPos,
        _inner_bitmap_state: &mut InnerBitmap,
        key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        key.store(&RestingOrder {
            maker: *msg_sender,
            base_lots,
        });

        Ok(base_lots)
    }

    fn decrease<M, B, Q>(
        _msg_sender: &Address,
        _base_lots: BaseLots,
        _inner_pos: InnerPos,
        _inner_bitmap_state: &mut InnerBitmap,
        _key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        unreachable!()
    }
}
