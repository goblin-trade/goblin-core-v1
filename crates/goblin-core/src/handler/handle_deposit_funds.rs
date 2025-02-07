use core::mem;

use crate::market_params::MarketParams;

pub fn handle_deposit_funds(input: &[u8]) {
    if input.len() < mem::size_of::<MarketParams>() {
        return;
    }
    let market_params = unsafe { &*(input.as_ptr() as *const MarketParams) };

    // Now you can use market_params directly
    // Example: process the market parameters
    let _ = market_params.base_token;
    let _ = market_params.quote_token;
    let _ = market_params.base_lot_size;
    let _ = market_params.quote_lot_size;
}
