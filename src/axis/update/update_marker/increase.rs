use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Increase},
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, Position, QuantityOps, Ticks, INNER_POS_V2},
    settlement::local_delta::LocalSenderDelta,
    state::{bitmap_v2::BitmapV2, resting_order::preimage::RestingOrderPreimage, Preimage},
    types::StoreReader,
};

impl UpdateMarker for Increase {
    fn process_update<M, B, Q, In>(
        local_sender_delta: &mut LocalSenderDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        position_2: Position,
        base_lots: BaseLots,
        _inner_bitmap_state: &mut BitmapV2<INNER_POS_V2>,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let resting_order_key = RestingOrderPreimage {
            market_key: market_and_key.market_key,
            position: position_2,
        }
        .hash();

        let mut resting_order_state = resting_order_key.load();
        resting_order_state.size = resting_order_state
            .size
            .checked_add(base_lots)
            .ok_or(GoblinError::Overflow)?;
        resting_order_key.store(&resting_order_state);

        // Update delta
        let base_lot_size = Base::get(&market_and_key.market.lot_size_pair);
        let price = Ticks::from(position_2);

        local_sender_delta.add_resting_order_deposit::<In>(
            base_lots,
            base_lot_size,
            market_and_key.market.tick_size,
            price,
        )
    }
}
