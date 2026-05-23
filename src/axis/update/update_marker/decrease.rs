use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease},
    },
    goblin_error::GoblinError,
    instructions::PosHeader,
    quantities::Ticks,
    require,
    state::{bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, Preimage},
    types::StoreReader,
};

impl UpdateMarker for Decrease {
    fn process_update<'a, M, B, Q, In>(
        readables: &Readables<M, B, Q>,
        PosHeader {
            position,
            base_lots,
        }: PosHeader,
        writables: &mut Writables,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let Readables {
            msg_sender,
            market_and_key,
        } = *readables;
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

        writables
            .local_delta
            .local_sender_delta
            .subtract_resting_order_deposit::<In>(
                reduced_lots,
                base_lot_size,
                market_and_key.market.tick_size,
                price,
            )
    }
}
