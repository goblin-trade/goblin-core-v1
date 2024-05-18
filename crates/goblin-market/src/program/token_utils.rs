use stylus_sdk::{
    alloy_primitives::{Address, U256},
    contract,
    stylus_proc::sol_interface,
};

use crate::{
    parameters::{BASE_TOKEN, QUOTE_TOKEN},
    program::error::GoblinResult,
    quantities::{BaseAtomsRaw, QuoteAtomsRaw},
    GoblinMarket,
};

sol_interface! {
    interface IERC20 {
        function balanceOf(address owner) external view returns (uint);

        function transfer(address recipient, uint256 amount) external returns (bool);

        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
    }
}

/// Withdraw token if the amount is greater than 0
pub fn maybe_invoke_withdraw(
    context: &mut GoblinMarket,
    withdraw_amount: U256,
    token_address: Address,
    recipient: Address,
) -> GoblinResult<()> {
    if withdraw_amount > U256::ZERO {
        let token = IERC20::new(token_address);
        token.transfer(context, recipient, withdraw_amount).unwrap();
    }

    Ok(())
}

/// Deposit token if the amount is greater than 0
pub fn maybe_invoke_deposit(
    context: &mut GoblinMarket,
    deposit_amount: U256,
    token_address: Address,
    trader: Address,
) -> GoblinResult<()> {
    if deposit_amount > U256::ZERO {
        let token = IERC20::new(token_address);
        token
            .transfer_from(context, trader, contract::address(), deposit_amount)
            .unwrap();
    }

    Ok(())
}

/// Withdraw base and quote tokens
pub fn try_withdraw(
    context: &mut GoblinMarket,
    quote_amount: QuoteAtomsRaw,
    base_amount: BaseAtomsRaw,
    recipient: Address,
) -> GoblinResult<()> {
    maybe_invoke_withdraw(context, quote_amount.as_u256(), QUOTE_TOKEN, recipient)?;
    maybe_invoke_withdraw(context, base_amount.as_u256(), BASE_TOKEN, recipient)?;

    Ok(())
}

/// Deposit base and quote tokens
pub fn try_deposit(
    context: &mut GoblinMarket,
    quote_amount: QuoteAtomsRaw,
    base_amount: BaseAtomsRaw,
    trader: Address,
) -> GoblinResult<()> {
    maybe_invoke_deposit(context, quote_amount.as_u256(), QUOTE_TOKEN, trader)?;
    maybe_invoke_deposit(context, base_amount.as_u256(), BASE_TOKEN, trader)?;

    Ok(())
}
