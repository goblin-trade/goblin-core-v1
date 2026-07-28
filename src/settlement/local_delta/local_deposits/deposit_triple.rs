use crate::{
    axis::token::{token_quantity::TokenQuantity, CustomERC20, HardcodedERC20, Token, ETH},
    types::Triple,
};

/// Local market deposits for a given side. The triple covers the 3 possible token
/// variants per side
///
pub type DepositTriple = Triple<
    <ETH as TokenQuantity>::LocalDeposit,
    <HardcodedERC20 as TokenQuantity>::LocalDeposit,
    <CustomERC20 as TokenQuantity>::LocalDeposit,
    Token,
>;
