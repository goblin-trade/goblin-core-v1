use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, CommonMarket},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Increase},
    },
    goblin_error::GoblinError,
    matching::bitmap::FullCoordinates,
    quantities::{BaseLots, QuantityOps, Ticks},
    settlement::local_delta::LocalSenderDelta,
    state::{
        bitmap::inner_bitmap::InnerBitmap, resting_order::preimage::RestingOrderPreimage, SlotKey,
    },
    types::StoreReader,
};

impl UpdateMarker for Increase {
    fn process_update<M, B, Q, In>(
        local_sender_delta: &mut LocalSenderDelta,
        market: &CommonMarket<M, B, Q>,
        resting_order_key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        full_coordinates: &FullCoordinates,
        base_lots: BaseLots,
        _inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let mut resting_order_state = resting_order_key.load();
        resting_order_state.size = resting_order_state
            .size
            .checked_add(base_lots)
            .ok_or(GoblinError::Overflow)?;
        resting_order_key.store(&resting_order_state);

        // Update delta
        let base_lot_size = Base::get(&market.lot_size_pair);
        let price = Ticks::from(*full_coordinates);
        local_sender_delta.add_resting_order_deposit::<In>(
            base_lots,
            base_lot_size,
            market.tick_size,
            price,
        )?;

        Ok(())
    }
}
