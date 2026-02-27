use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Leg, Quote},
        market::{market_marker::MarketMarker, CommonMarket, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::{
        active_iterator::resting_order::{quote_pair::QuotePair, RestingOrderIterator},
        bitmap::{range::Range, FullCoordinates},
    },
    settlement::local_delta::{LocalMakerDeltas, MakerDelta, TakerDelta},
    types::{Address, StoreReader, Tuple},
};

/// Iterator that matches taker budget against resting orders
/// Continue matching till budget is exhausted or price limit is reached
pub struct BudgetIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher
        + StoreReader<Tuple<MakerDelta<Base>, MakerDelta<Quote>, Leg>, Result = MakerDelta<In>>
        + StoreReader<Tuple<TakerDelta<Base>, TakerDelta<Quote>, Leg>, Result = TakerDelta<In>>,
{
    pub taker: &'a Address,
    pub market: &'a CommonMarket<M, B, Q>,
    pub budget: In::MatchingLots,
    pub resting_order_iterator: RestingOrderIterator<'a, M, B, Q, In>,
    pub taker_delta: TakerDelta<In>,
    pub local_maker_deltas: &'a mut LocalMakerDeltas,
}

impl<'a, M, B, Q, In> BudgetIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher
        + StoreReader<Tuple<MakerDelta<Base>, MakerDelta<Quote>, Leg>, Result = MakerDelta<In>>
        + StoreReader<Tuple<TakerDelta<Base>, TakerDelta<Quote>, Leg>, Result = TakerDelta<In>>,
{
    pub fn new(
        taker: &'a Address,
        market_and_key: &'a MarketAndKey<M, B, Q>,
        coordinates_range: Range<FullCoordinates<In>>,
        num_lots: In::Lots,
        local_maker_deltas: &'a mut LocalMakerDeltas,
    ) -> Result<Self, GoblinError> {
        let MarketAndKey {
            market,
            key: market_key,
        } = market_and_key;

        let base_lot_size = Base::get(&market.lot_size_pair);
        let budget = In::matching_lots_taker(num_lots, base_lot_size);

        Ok(Self {
            taker,
            market,
            budget,
            resting_order_iterator: RestingOrderIterator::new(market_key, coordinates_range)?,
            taker_delta: TakerDelta::<In>::zero(),
            local_maker_deltas,
        })
    }

    pub fn add_quote(&mut self, maker: Address, quote_pair: QuotePair<In>) -> Option<()> {
        // Update taker
        self.taker_delta.matched_lots.checked_add_v2(quote_pair)?;

        // Update maker
        let maker_delta_pair = self.local_maker_deltas.get_or_insert_mut(maker)?;
        let maker_delta = In::get_leg_mut(maker_delta_pair);
        maker_delta.matched_lots.checked_add_v2(quote_pair)?;

        Some(())
    }
}
