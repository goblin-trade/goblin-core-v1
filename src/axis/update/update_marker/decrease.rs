use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Decrease},
    },
    goblin_error::GoblinError,
    instructions::{MakeReadables, PosHeader},
    quantities::Ticks,
    require,
    state::{bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, Preimage},
};

impl UpdateMarker for Decrease {
    fn process_update<'a, M, B, Q, In>(
        make_readables: &MakeReadables<M, B, Q>,
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
        } = *make_readables.readables;

        let PosHeader {
            position,
            base_lots,
        } = make_readables.pos_header;

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
        let price = Ticks::from(position);
        let delta = In::get_leg_mut(&mut writables.local_delta.local_sender_delta);
        Self::update_make_delta::<In>(
            &mut delta.make,
            reduced_lots,
            market_and_key.market.tick_size,
            price,
        )
    }
}
