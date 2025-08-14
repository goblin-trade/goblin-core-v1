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

    // Halt early if best price is further from the centre than the price limit
    if market_state.price_limit_reached_v2::<S>(price_limit) {
        return Ok(());
    }

    let mut quote_iterator =
        QuoteIterator::<S::Opposite>::new(market_state.best_price_mut_v2::<S::Opposite>());

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
