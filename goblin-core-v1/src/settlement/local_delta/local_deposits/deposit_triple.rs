use crate::{
    axis::token::{token_quantity::TokenQuantity, CustomERC20, HardcodedERC20, Token, ETH},
    types::Triple,
};

/// Deposit amounts stored in local delta which get committed in global delta.
/// The triple covers the 3 possible token variants per leg
pub type DepositTriple = Triple<
    <ETH as TokenQuantity>::LocalDeposit,
    <HardcodedERC20 as TokenQuantity>::LocalDeposit,
    <CustomERC20 as TokenQuantity>::LocalDeposit,
    Token,
>;
