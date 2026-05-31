use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
        update::Update,
    },
    goblin_error::GoblinError,
    instructions::{MakeReadables, PosHeader},
    quantities::{BaseLots, InnerPos},
    require,
    settlement::CheckedAdd,
    state::{
        bitmap::alias::InnerBitmap,
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        Preimage, SlotKey,
    },
    types::{StoreReader, Tuple},
};

pub trait UpdateMarker {
    fn update_resting_order<'a, M, B, Q, In>(
        resting_order_key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        base_lots: BaseLots,
        inner_pos: InnerPos,
        resting_order_state: &mut RestingOrder,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;

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
        Self: StoreReader<
            Tuple<
                <In::Opposite as LegMatcher>::MatchingLots,
                <In::Opposite as LegMatcher>::MatchingLots,
                Update,
            >,
            Result = <In::Opposite as LegMatcher>::MatchingLots,
        >,
    {
        let Readables {
            msg_sender,
            market_readables,
        } = *make_readables.readables;

        let PosHeader {
            position,
            base_lots,
        } = make_readables.pos_header;

        let resting_order_key = RestingOrderPreimage {
            market_key: market_readables.market_key,
            position,
        }
        .hash();

        let mut resting_order_state = resting_order_key.load();
        require!(
            resting_order_state.maker == *msg_sender,
            GoblinError::UnauthorizedMsgSender
        );

        let updated_base_lots = Self::update_resting_order::<M, B, Q, In>(
            &resting_order_key,
            base_lots,
            position.into(),
            &mut resting_order_state,
            inner_bitmap_state,
        )?;

        let amount = <In::Opposite as LegMatcher>::matching_lots_maker(
            updated_base_lots,
            market_readables.market.tick_size,
            position.into(),
        );

        let sided_make_delta = In::get_leg_mut(&mut writables.local_delta.local_sender_delta);
        let delta = Self::get_leg_mut(&mut sided_make_delta.make);
        *delta = delta.checked_add(amount).ok_or(GoblinError::Overflow)?;

        Ok(())
    }
}
