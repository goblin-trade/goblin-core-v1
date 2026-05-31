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
        bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, KeyValue,
        Preimage,
    },
    types::{StoreReader, Tuple},
};

pub trait UpdateMarker<In>
where
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
    fn update_resting_order<'a, M, B, Q>(
        base_lots: BaseLots,
        inner_pos: InnerPos,
        inner_bitmap_state: &mut InnerBitmap,
        key_value: &mut KeyValue<RestingOrderPreimage<M, B, Q>>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker;

    fn process_update<'a, M, B, Q>(
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
            market_readables,
        } = *make_readables.readables;

        let PosHeader {
            position,
            base_lots,
        } = make_readables.pos_header;

        let key_value = &mut RestingOrderPreimage {
            market_key: market_readables.market_key,
            position,
        }
        .key_value();

        require!(
            key_value.value.maker == *msg_sender,
            GoblinError::UnauthorizedMsgSender
        );

        let updated_base_lots = Self::update_resting_order::<M, B, Q>(
            base_lots,
            position.into(),
            inner_bitmap_state,
            key_value,
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
