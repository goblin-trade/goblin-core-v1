use core::ops::Add;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    contract,
    stylus_proc::sol_interface,
};

use crate::{
    error::GoblinResult,
    parameters::{BASE_TOKEN, QUOTE_TOKEN},
    GoblinMarket,
};

sol_interface! {
    interface IERC20 {
        function balanceOf(address owner) external view returns (uint);

        function transfer(address recipient, uint256 amount) external returns (bool);

        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
    }
}

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

pub fn try_deposit(
    context: &mut GoblinMarket,
    quote_amount: U256,
    base_amount: U256,
    trader: Address,
) -> GoblinResult<()> {
    maybe_invoke_deposit(context, quote_amount, QUOTE_TOKEN, trader)?;
    maybe_invoke_deposit(context, base_amount, BASE_TOKEN, trader)?;

    Ok(())
}
