use crate::{
    axis::LegMatcher,
    axis_helpers::TokenPair,
    matching::RestingOrderEntry,
    quantities::{QuoteLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks},
    state::RestingOrder,
    types::Address,
};

/// Result of matching against a single resting order
pub struct FillOutcome<'a, In: LegMatcher> {
    pub counterparty: &'a Address,
    pub price_in_quote_lots: QuoteLotsPerBaseUnit,
    pub matched: In::MatchingLots,
    pub budget_exhausted: bool,
}

impl<'a, In: LegMatcher> FillOutcome<'a, In> {
    pub fn new<TP: TokenPair>(
        tick_size: QuoteLotsPerBaseUnitPerTick,
        RestingOrderEntry {
            position,
            resting_order_key_value,
        }: &'a RestingOrderEntry<TP>,
        budget: &mut In::MatchingLots,
    ) -> Self {
        let counterparty = &resting_order_key_value.value.maker;
        let price_in_quote_lots = tick_size * Ticks::from(*position);

        let quote =
            In::matching_lots_maker(resting_order_key_value.value.base_lots, price_in_quote_lots);
        let matched = quote.min(*budget);

        *budget -= matched;
        let budget_exhausted = *budget == In::MatchingLots::default();

        if budget_exhausted {
            let residue = quote - matched;

            // Write resting order with residue to slot
            if residue > In::MatchingLots::default() {
                let base_lots = In::base_lots_maker(residue, price_in_quote_lots);
                let updated_resting_order = RestingOrder {
                    maker: *counterparty,
                    base_lots,
                };

                resting_order_key_value.key.store(&updated_resting_order);
            }
        }

        Self {
            counterparty,
            price_in_quote_lots,
            matched,
            budget_exhausted,
        }
    }
}
