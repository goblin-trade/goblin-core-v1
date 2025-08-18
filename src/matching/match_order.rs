use crate::{
    goblin_error::GoblinError,
    markets::IndexedMarket,
    matching::quote_iterator::{Quote, QuoteIterator},
    quantities::Ticks,
    require,
    state::MarketState,
    types::SideMarker,
};

pub struct MatchResult<S: SideMarker> {
    pub lots_in: S::Lots,
    pub lots_out: <S::Opposite as SideMarker>::Lots,
}

pub fn match_order<S: SideMarker>(
    indexed_market: &IndexedMarket,
    market_state: &mut MarketState,
    num_lots: S::Lots,
    min_lots_to_fill: S::Lots,
    price_limit: Ticks,
) -> Result<MatchResult<S>, GoblinError> {
    let budget = S::get_budget(num_lots, indexed_market.base_lot_size);

    let mut matched = S::Quote::from(0);
    let mut matched_opposite = <S::Opposite as SideMarker>::Quote::from(0);

    // Halt early if best price is further from the centre than the price limit
    let best_opposite_price = S::Opposite::best_price_mut(market_state);
    if S::Opposite::closer_to_centre(*best_opposite_price, price_limit) {
        require!(
            min_lots_to_fill == S::Lots::from(0),
            GoblinError::TakerPriceLimitReached
        );

        return Ok(MatchResult {
            lots_in: S::Lots::from(0),
            lots_out: <S::Opposite as SideMarker>::Lots::from(0),
        });
    }

    let mut quote_iterator = QuoteIterator::<S::Opposite>::new(best_opposite_price);

    // TODO separate price iteration from resting order reads
    // If price is beyond threshold no need to read resting order from slot
    while let Some(Quote {
        price,
        resting_order,
    }) = quote_iterator.next()
    {
        // Get quote in both forms and add to accumulators
        let quote = S::get_resting_order_quote(resting_order.size, indexed_market.tick_size, price);

        if (matched + quote) > budget {
            let amount_to_add = budget - matched;

            // Amount completely filled
            matched = budget;
            matched_opposite +=
                S::get_opposite_quote(amount_to_add, indexed_market.tick_size, price);

            break;
        }

        matched += quote;
        matched_opposite += S::Opposite::get_resting_order_quote(
            resting_order.size,
            indexed_market.tick_size,
            price,
        );
    }

    let matched_lots = S::get_lots_from_quote(matched, indexed_market.base_lot_size);
    require!(
        matched_lots >= min_lots_to_fill,
        GoblinError::InsufficientTakerFill
    );
    let matched_lots_opposite =
        S::Opposite::get_lots_from_quote(matched_opposite, indexed_market.base_lot_size);

    // Update deltas
    Ok(MatchResult {
        lots_in: matched_lots,
        lots_out: matched_lots_opposite,
    })
}
