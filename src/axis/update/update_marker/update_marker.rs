use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, Readables, Writables},
        occupancy::occupancy_marker::OccupancyMarker,
        token::token_marker::TokenMarker,
        update::Update,
    },
    goblin_error::GoblinError,
    instructions::{MakeReadables, PosHeader},
    quantities::{BaseLots, InnerPos},
    settlement::CheckedAdd,
    state::{
        bitmap::alias::InnerBitmap, resting_order::preimage::RestingOrderPreimage, Preimage,
        SlotKey,
    },
    types::{Address, StoreReader, Tuple},
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
    fn update_resting_order<'a, M, B, Q, Oc>(
        msg_sender: &Address,
        base_lots: BaseLots,
        inner_pos: InnerPos,
        inner_bitmap_state: &mut InnerBitmap,
        key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
    ) -> Result<BaseLots, GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        Oc: OccupancyMarker;

    fn process_update<'a, M, B, Q, Oc>(
        make_readables: &MakeReadables<M, B, Q>,
        writables: &mut Writables,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
        Oc: OccupancyMarker,
    {
        let Readables {
            msg_sender,
            market_readables,
        } = *make_readables.readables;

        let PosHeader {
            position,
            base_lots,
        } = make_readables.pos_header;

        let key = &mut &mut RestingOrderPreimage {
            market_key: market_readables.market_key,
            position,
        }
        .hash();

        let updated_base_lots = Self::update_resting_order::<M, B, Q, Oc>(
            msg_sender,
            base_lots,
            position.into(),
            inner_bitmap_state,
            key,
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
