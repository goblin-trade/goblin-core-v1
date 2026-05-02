use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, CommonMarket},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease},
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, Position, Ticks, INNER_POS_V2},
    settlement::local_delta::LocalSenderDelta,
    state::{bitmap_v2::BitmapV2, resting_order::preimage::RestingOrderPreimage, SlotKey},
    types::StoreReader,
};

impl UpdateMarker for Decrease {
    fn process_update<M, B, Q, In>(
        local_sender_delta: &mut LocalSenderDelta,
        market: &CommonMarket<M, B, Q>,
        resting_order_key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        position_2: Position,
        base_lots: BaseLots,
        inner_bitmap_state: &mut BitmapV2<INNER_POS_V2>,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let mut resting_order_state = resting_order_key.load();

        let reduced_lots = if resting_order_state.size > base_lots {
            resting_order_state.size -= base_lots;
            resting_order_key.store(&resting_order_state);

            base_lots
        } else {
            inner_bitmap_state.deactivate(position_2.into());

            resting_order_state.size
        };

        // Update delta
        let base_lot_size = Base::get(&market.lot_size_pair);
        let price = Ticks::from(position_2);

        local_sender_delta.subtract_resting_order_deposit::<In>(
            reduced_lots,
            base_lot_size,
            market.tick_size,
            price,
        )
    }
}
