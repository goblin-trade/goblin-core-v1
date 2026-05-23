use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Increase},
    },
    goblin_error::GoblinError,
    instructions::PosHeader,
    quantities::{QuantityOps, Ticks},
    require,
    state::{bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, Preimage},
    types::StoreReader,
};

impl UpdateMarker for Increase {
    fn process_update<'a, M, B, Q, In>(
        readables: &Readables<M, B, Q>,
        PosHeader {
            position,
            base_lots,
        }: PosHeader,
        writables: &mut Writables,
        _inner_bitmap_state: &mut InnerBitmap,
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

        resting_order_state.base_lots = resting_order_state
            .base_lots
            .checked_add(base_lots)
            .ok_or(GoblinError::Overflow)?;
        resting_order_key.store(&resting_order_state);

        // Update delta
        let base_lot_size = Base::get(&market_and_key.market.lot_size_pair);
        let price = Ticks::from(position);

        writables
            .local_delta
            .local_sender_delta
            .add_resting_order_deposit::<In>(
                base_lots,
                base_lot_size,
                market_and_key.market.tick_size,
                price,
            )
    }
}
