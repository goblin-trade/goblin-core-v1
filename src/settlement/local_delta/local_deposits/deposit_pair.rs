use crate::{
    axis::{leg::Pair, token::token_quantity::TokenQuantity},
    settlement::ConstZero,
};

pub type DepositPair<B, Q> =
    Pair<<B as TokenQuantity>::LocalDeposit, <Q as TokenQuantity>::LocalDeposit>;

// TODO implement ConstZero and Decode generically on Tuple
// impl<B, Q> ConstZero for DepositPair<B, Q>
// where
//     B: TokenQuantity,
//     Q: TokenQuantity,
// {
//     const ZEROED: Self = Pair::new(
//         <B as TokenQuantity>::LocalDeposit::ZEROED,
//         <Q as TokenQuantity>::LocalDeposit::ZEROED,
//     );
// }
