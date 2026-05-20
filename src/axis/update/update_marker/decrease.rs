use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease},
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, Position, Ticks, INNER_POS, POS_1},
    require,
    settlement::local_delta::LocalSenderDelta,
    state::{bitmap::Bitmap, resting_order::preimage::RestingOrderPreimage, Preimage},
    types::{Address, StoreReader},
};

impl UpdateMarker for Decrease {
    fn process_update<M, B, Q, In>(
        msg_sender: &Address,
        local_sender_delta: &mut LocalSenderDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        position: Position,
        base_lots: BaseLots,
        inner_bitmap_state: &mut Bitmap<POS_1, INNER_POS>,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let resting_order_key = RestingOrderPreimage {
            market_key: market_and_key.market_key,
            position,
        }
        .hash();

        let mut resting_order_state = resting_order_key.load();
        require!(
            resting_order_state.maker == *msg_sender,
            GoblinError::UnauthorizedMsgSender
        );

        let reduced_lots = if resting_order_state.base_lots > base_lots {
            resting_order_state.base_lots -= base_lots;
            resting_order_key.store(&resting_order_state);

            base_lots
        } else {
            inner_bitmap_state.deactivate(position.into());

            resting_order_state.base_lots
        };

        // Update delta
        let base_lot_size = Base::get(&market_and_key.market.lot_size_pair);
        let price = Ticks::from(position);

        local_sender_delta.subtract_resting_order_deposit::<In>(
            reduced_lots,
            base_lot_size,
            market_and_key.market.tick_size,
            price,
        )
    }
}
