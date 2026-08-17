use crate::{
    axis::{leg::Pair, token::token_quantity::TokenQuantity},
    market::TokenPair,
};

/// Deposit amounts read from calldata
pub type DepositPair<TP> = Pair<
    <<TP as TokenPair>::Base as TokenQuantity>::LocalDeposit,
    <<TP as TokenPair>::Quote as TokenQuantity>::LocalDeposit,
>;
