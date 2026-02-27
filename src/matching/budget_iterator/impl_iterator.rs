use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Leg, Quote},
        market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::{
        active_iterator::resting_order::{quote_pair::QuotePair, RestingOrderIterator},
        bitmap::range::Range,
        budget_iterator::BudgetIterator,
    },
    quantities::{QuantityOps, Ticks},
    settlement::local_delta::{MakerDelta, TakerDelta},
    state::{MarketPreimage, SlotKey},
    types::{StoreReader, Tuple},
};

impl<'a, M, B, Q, In> Iterator for BudgetIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher
        + StoreReader<Tuple<MakerDelta<Base>, MakerDelta<Quote>, Leg>, Result = MakerDelta<In>>
        + StoreReader<Tuple<TakerDelta<Base>, TakerDelta<Quote>, Leg>, Result = TakerDelta<In>>,
{
    type Item = ();

    // Each next() call corresponds to reading a new resting order
    fn next(&mut self) -> Option<Self::Item> {
        if self.budget == In::MatchingLots::ZERO {
            return None;
        }

        if let Some(item) = self.resting_order_iterator.next() {
            let maker_quote = item.quote(self.market.tick_size);

            // Self trade
            if item.resting_order.maker == *self.taker {
                self.taker_delta.taker_self_trade_unlocked += maker_quote.quote_opposite;

                return Some(());
            }

            // TODO simpler utility
            let consumed = self.budget.min(maker_quote.quote);
            let consumed_base_lots =
                In::base_lots_from_matching(consumed, self.market.tick_size, item.price());
            let consumed_opposite = In::Opposite::matching_lots_maker(
                consumed_base_lots,
                self.market.tick_size,
                item.price(),
            );

            self.add_quote(
                item.resting_order.maker,
                QuotePair {
                    quote: consumed,
                    quote_opposite: consumed_opposite,
                },
            )?;
        }

        None
    }
}
