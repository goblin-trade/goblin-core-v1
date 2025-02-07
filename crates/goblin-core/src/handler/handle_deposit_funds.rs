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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_params::MarketParams;
    use crate::{clear_state, hostio::*, selector, user_entrypoint};

    #[test]
    fn test_deposit_funds() {
        // Clear any previous test state
        clear_state();

        // Create test input
        let market_params = MarketParams {
            base_token: [0u8; 20],
            quote_token: [1u8; 20],
            base_lot_size: 1,
            quote_lot_size: 2,
        };

        let mut input = vec![selector::DEPOSIT_FUNDS_SELECTOR];
        input.extend_from_slice(unsafe {
            core::slice::from_raw_parts(
                &market_params as *const MarketParams as *const u8,
                core::mem::size_of::<MarketParams>(),
            )
        });

        // Set the test input
        set_test_args(input.clone());

        // Call the contract entrypoint
        let result = user_entrypoint(input.len());

        // Assert the result
        assert_eq!(result, 0);

        // Verify logs
        let logs = get_logs();
        println!("logs {:?}", logs);
    }

    #[test]
    fn test_invalid_selector() {
        clear_state();

        // Test with invalid selector
        let input = vec![0xFF];
        set_test_args(input.clone());

        let result = user_entrypoint(input.len());
        assert_eq!(result, 1);
    }
}
