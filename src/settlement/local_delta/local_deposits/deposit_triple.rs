use crate::{
    axis::token::{token_quantity::TokenQuantity, CustomERC20, HardcodedERC20, Token, ETH},
    settlement::ConstZero,
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

impl ConstZero for DepositTriple {
    const ZEROED: Self = Triple::new(
        <ETH as TokenQuantity>::LocalDeposit::ZEROED,
        <HardcodedERC20 as TokenQuantity>::LocalDeposit::ZEROED,
        <CustomERC20 as TokenQuantity>::LocalDeposit::ZEROED,
    );
}
