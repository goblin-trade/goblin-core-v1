use crate::{
    goblin_error::GoblinError,
    matching::quote_iterator::{Quote, QuoteIterator},
    quantities::{BaseLots, QuoteLotsPerBaseUnitPerTick, Ticks},
    require,
    state::MarketState,
    types::{Side, SideMarker},
};

pub fn match_order_v2<S: SideMarker>(
    market_state: &mut MarketState,
    tick_size: QuoteLotsPerBaseUnitPerTick,
    num_lots: S::Quote,
    min_lots_to_fill: S::Quote,
    price_limit: Ticks,
) -> Result<(), GoblinError>
where
    <S as SideMarker>::Quote: core::ops::SubAssign,
{
    let mut initial_budget = num_lots;

    let side = S::side();
    let opposite_side = side.opposite();

    // Halt early if best price is further from the centre than the price limit
    if market_state.price_limit_reached(side, price_limit) {
        return Ok(());
    }

    let mut quote_iterator = QuoteIterator {
        side: opposite_side,
        best_price: market_state.best_price_mut(opposite_side),
    };

    while let Some(Quote {
        price,
        resting_order,
    }) = quote_iterator.next()
    {
        let quote = S::get_quote(resting_order.size, tick_size, price);
        initial_budget -= quote;
    }

    Ok(())
}

// pub fn match_order(
//     market_state: &mut MarketState,
//     side: Side,
//     num_lots: u64,
//     min_lots_to_fill: u64,
//     price_limit: Ticks,
// ) -> Result<(), GoblinError> {
//     let opposite_side = side.opposite();

//     // Halt early if best price is further from the centre than the price limit
//     if market_state.price_limit_reached(side, price_limit) {
//         return Ok(());
//     }

//     // Read quotes one by one
//     // * Read outer index matrix
//     // * Read inner index matrix
//     // * Read trader state- this is separate from the matrix logic
//     //
//     // Calling next() again will close the previous slot
//     // Each quote will deduct from budget

//     let mut match_iterator = MatchIterator {
//         side: opposite_side,
//         best_price: market_state.best_price_mut(opposite_side),
//     };

//     while let Some(resting_order) = match_iterator.next() {}

//     Ok(())
// }
