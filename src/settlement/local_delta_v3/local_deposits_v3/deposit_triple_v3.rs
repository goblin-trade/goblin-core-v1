use crate::{
    axis::{
        leg::leg_quantities::LegQuantities,
        token::{token_marker::TokenMarker, CustomERC20, HardcodedERC20, Token, ETH},
    },
    settlement::ConstZero,
    types::Triple,
};

/// Local market deposits for a given side. The triple covers the 3 possible token
/// variants per side
pub struct DepositTripleV3<In: LegQuantities>(
    Triple<
        <ETH as TokenMarker>::LocalDeposit<In>,
        <HardcodedERC20 as TokenMarker>::LocalDeposit<In>,
        <CustomERC20 as TokenMarker>::LocalDeposit<In>,
        Token,
    >,
);

impl<In> ConstZero for DepositTripleV3<In>
where
    In: LegQuantities,
{
    const ZEROED: Self = Self(Triple::new(
        <ETH as TokenMarker>::LocalDeposit::<In>::ZEROED,
        <HardcodedERC20 as TokenMarker>::LocalDeposit::<In>::ZEROED,
        <CustomERC20 as TokenMarker>::LocalDeposit::<In>::ZEROED,
    ));
}
