use crate::{
    axis::token::{token_marker::TokenMarker, CustomERC20, HardcodedERC20, Token, ETH},
    settlement::ConstZero,
    types::Triple,
};

/// Local market deposits for a given side. The triple covers the 3 possible token
/// variants per side
///
pub type DepositTriple = Triple<
    <ETH as TokenMarker>::LocalDeposit,
    <HardcodedERC20 as TokenMarker>::LocalDeposit,
    <CustomERC20 as TokenMarker>::LocalDeposit,
    Token,
>;

impl ConstZero for DepositTriple {
    const ZEROED: Self = Triple::new(
        <ETH as TokenMarker>::LocalDeposit::ZEROED,
        <HardcodedERC20 as TokenMarker>::LocalDeposit::ZEROED,
        <CustomERC20 as TokenMarker>::LocalDeposit::ZEROED,
    );
}
