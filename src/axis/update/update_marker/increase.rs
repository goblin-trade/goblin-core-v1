use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, Increase},
    },
    goblin_error::GoblinError,
    instructions::{MakeReadables, PosHeader},
    quantities::Ticks,
    require,
    settlement::CheckedAdd,
    state::{bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, Preimage},
};

impl UpdateMarker for Increase {
    fn process_update<'a, M, B, Q, In>(
        make_readables: &MakeReadables<M, B, Q>,
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

        resting_order_state.base_lots = resting_order_state
            .base_lots
            .checked_add(base_lots)
            .ok_or(GoblinError::Overflow)?;
        resting_order_key.store(&resting_order_state);

        // Update delta
        let price = Ticks::from(position);
        let amount = <In::Opposite as LegMatcher>::matching_lots_maker(
            base_lots,
            market_and_key.market.tick_size,
            price,
        );

        let delta = In::get_leg_mut(&mut writables.local_delta.local_sender_delta);

        // TODO use generic getter to fetch increase & decrease fields
        // MakeDelta should use Tuple
        delta.make.increase = delta
            .make
            .increase
            .checked_add(amount)
            .ok_or(GoblinError::Overflow)?;

        Ok(())
    }
}
