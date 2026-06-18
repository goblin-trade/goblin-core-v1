use crate::{
    axis::token::{token_reader::TokenReader, CustomERC20, HardcodedERC20, Token, ETH},
    settlement::ConstZero,
    types::Triple,
};

/// Local market deposits for a given side. The triple covers the 3 possible token
/// variants per side
pub type DepositTriple = Triple<
    <ETH as TokenReader>::Deposit,
    <HardcodedERC20 as TokenReader>::Deposit,
    <CustomERC20 as TokenReader>::Deposit,
    Token,
>;

impl ConstZero for DepositTriple {
    const ZEROED: Self = Triple::new(
        <ETH as TokenReader>::Deposit::ZEROED,
        <HardcodedERC20 as TokenReader>::Deposit::ZEROED,
        <CustomERC20 as TokenReader>::Deposit::ZEROED,
    );
}
