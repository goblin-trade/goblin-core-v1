use crate::{
    goblin_error::GoblinError,
    matching::quote_iterator::{Quote, QuoteIterator},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, Ticks},
    state::MarketState,
    types::SideMarker,
};

pub fn match_order<S: SideMarker>(
    market_state: &mut MarketState,
    tick_size: QuoteLotsPerBaseUnitPerTick,
    base_lot_size: BaseLotsPerBaseUnit,
    num_lots: S::Lots,
    min_lots_to_fill: S::Lots,
    price_limit: Ticks,
) -> Result<(), GoblinError>
where
    <S as SideMarker>::Quote: core::ops::SubAssign,
    <S as SideMarker>::Opposite: SideMarker,
{
    let mut initial_budget = S::get_budget(num_lots, base_lot_size);

    let best_opposite_price = S::Opposite::best_price_mut(market_state);

    // Halt early if best price is further from the centre than the price limit
    if S::Opposite::closer_to_centre(*best_opposite_price, price_limit) {
        return Ok(());
    }

    let mut quote_iterator = QuoteIterator::<S::Opposite>::new(best_opposite_price);

    while let Some(Quote {
        price,
        resting_order,
    }) = quote_iterator.next()
    {
        let quote = S::get_resting_order_quote(resting_order.size, tick_size, price);
        initial_budget -= quote;
    }

    Ok(())
}
