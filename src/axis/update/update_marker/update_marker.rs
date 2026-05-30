use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, Writables},
        token::token_marker::TokenMarker,
        update::Update,
    },
    goblin_error::GoblinError,
    instructions::MakeReadables,
    quantities::{BaseLots, QuoteLotsPerBaseUnitPerTick, Ticks},
    settlement::{CheckedAdd, SidedMakeDeltaV2},
    state::bitmap::alias::InnerBitmap,
    types::{StoreReader, Tuple},
};

pub trait UpdateMarker {
    fn process_update<'a, M, B, Q, In>(
        make_readables: &MakeReadables<M, B, Q>,
        writables: &mut Writables,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;

    fn update_make_delta<In>(
        sided_make_delta: &mut SidedMakeDeltaV2<In>,
        base_lots: BaseLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> Result<(), GoblinError>
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
        let amount = <In::Opposite as LegMatcher>::matching_lots_maker(base_lots, tick_size, price);

        let delta = Self::get_leg_mut(sided_make_delta);
        *delta = delta.checked_add(amount).ok_or(GoblinError::Overflow)?;

        Ok(())
    }
}
