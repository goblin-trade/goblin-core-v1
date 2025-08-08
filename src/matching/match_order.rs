use crate::{
    goblin_error::GoblinError, quantities::Ticks, require, state::MarketState, types::Side,
};

pub fn match_order(
    market_state: &mut MarketState,
    side: Side,
    num_lots: u64,
    price_limit: Ticks,
) -> Result<(), GoblinError> {
    let opposite_side = side.opposite();

    // Halt early if best price is further from the centre than the price limit
    if market_state.price_limit_reached(side, price_limit) {
        return Ok(());
    }

    // Read quotes one by one
    // * Read outer index matrix
    // * Read inner index matrix
    // * Read trader state- this is separate from the matrix logic
    //
    // Calling next() again will close the previous slot
    // Each quote will deduct from budget

    Ok(())
}
