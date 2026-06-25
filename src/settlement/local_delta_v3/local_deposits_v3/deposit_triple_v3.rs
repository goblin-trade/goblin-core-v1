use crate::{
    axis::{
        leg::{leg_quantities::LegQuantities, Base, Quote},
        token::{token_marker::TokenMarker, CustomERC20, HardcodedERC20, Token, ETH},
    },
    settlement::ConstZero,
    types::Triple,
};

/// Local market deposits for a given side. The triple covers the 3 possible token
/// variants per side
///
pub type DepositTripleV3<In: LegQuantities> = Triple<
    <ETH as TokenMarker>::LocalDeposit<In>,
    <HardcodedERC20 as TokenMarker>::LocalDeposit<In>,
    <CustomERC20 as TokenMarker>::LocalDeposit<In>,
    Token,
>;

impl ConstZero for DepositTripleV3<Base> {
    const ZEROED: Self = Triple::new(
        <ETH as TokenMarker>::LocalDeposit::<Base>::ZEROED,
        <HardcodedERC20 as TokenMarker>::LocalDeposit::<Base>::ZEROED,
        <CustomERC20 as TokenMarker>::LocalDeposit::<Base>::ZEROED,
    );
}

impl ConstZero for DepositTripleV3<Quote> {
    const ZEROED: Self = Triple::new(
        <ETH as TokenMarker>::LocalDeposit::<Quote>::ZEROED,
        <HardcodedERC20 as TokenMarker>::LocalDeposit::<Quote>::ZEROED,
        <CustomERC20 as TokenMarker>::LocalDeposit::<Quote>::ZEROED,
    );
}

// /// Local market deposits for a given side. The triple covers the 3 possible token
// /// variants per side
// ///
// /// Problem- we need wrapper struct to store In, but LegReader needs the underlying struct
// #[derive(Clone, Copy)]
// pub struct DepositTripleV3<In: LegQuantities>(
//     Triple<
//         <ETH as TokenMarker>::LocalDeposit<In>,
//         <HardcodedERC20 as TokenMarker>::LocalDeposit<In>,
//         <CustomERC20 as TokenMarker>::LocalDeposit<In>,
//         Token,
//     >,
// );

// impl<In> ConstZero for DepositTripleV3<In>
// where
//     In: LegQuantities,
// {
//     const ZEROED: Self = Self(Triple::new(
//         <ETH as TokenMarker>::LocalDeposit::<In>::ZEROED,
//         <HardcodedERC20 as TokenMarker>::LocalDeposit::<In>::ZEROED,
//         <CustomERC20 as TokenMarker>::LocalDeposit::<In>::ZEROED,
//     ));
// }
