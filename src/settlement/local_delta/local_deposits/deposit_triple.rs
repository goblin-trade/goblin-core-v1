use crate::{
    axis::token::{token_deltas::TokenDeltas, CustomERC20, HardcodedERC20, Token, ETH},
    settlement::ConstZero,
    types::Triple,
};

/// Local market deposits for a given side. The triple covers the 3 possible token
/// variants per side
///
pub type DepositTriple = Triple<
    <ETH as TokenDeltas>::LocalDeposit,
    <HardcodedERC20 as TokenDeltas>::LocalDeposit,
    <CustomERC20 as TokenDeltas>::LocalDeposit,
    Token,
>;

impl ConstZero for DepositTriple {
    const ZEROED: Self = Triple::new(
        <ETH as TokenDeltas>::LocalDeposit::ZEROED,
        <HardcodedERC20 as TokenDeltas>::LocalDeposit::ZEROED,
        <CustomERC20 as TokenDeltas>::LocalDeposit::ZEROED,
    );
}
