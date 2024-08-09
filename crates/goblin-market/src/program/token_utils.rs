use stylus_sdk::{
    alloy_primitives::{Address, U256},
    contract,
    stylus_proc::sol_interface,
};

use crate::{
    parameters::{BASE_TOKEN, QUOTE_TOKEN},
    program::error::GoblinResult,
    quantities::{BaseAtomsRaw, BaseLots, QuoteAtomsRaw, QuoteLots},
    GoblinMarket,
};

sol_interface! {
    interface IERC20 {
        function balanceOf(address owner) external view returns (uint);

        function transfer(address recipient, uint256 amount) external returns (bool);

        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);

        function allowance(address owner, address spender) external view returns (uint256);
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

/// ERC20 balance available to Goblin
/// This is MIN(balance, allowance)
///
/// # Arguments
///
/// * `token_address`
/// * `trader`
///
pub fn get_available_balance(
    context: &GoblinMarket,
    token_address: Address,
    trader: Address,
) -> U256 {
    let token = IERC20::new(token_address);
    let allowance = token
        .allowance(context, trader, contract::address())
        .unwrap();
    let balance = token.balance_of(context, trader).unwrap();

    allowance.min(balance)
}

pub fn get_available_base_lots(context: &GoblinMarket, trader: Address) -> BaseLots {
    let available_balance =
        BaseAtomsRaw::from_u256(get_available_balance(context, BASE_TOKEN, trader));
    available_balance.to_lots()
}

pub fn get_available_quote_lots(context: &GoblinMarket, trader: Address) -> QuoteLots {
    let available_balance =
        QuoteAtomsRaw::from_u256(get_available_balance(context, QUOTE_TOKEN, trader));
    available_balance.to_lots()
}
