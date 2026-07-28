use crate::axis::{leg::Pair, token::token_quantity::TokenQuantity};

/// Deposit amounts read from calldata
pub type DepositPair<B, Q> =
    Pair<<B as TokenQuantity>::LocalDeposit, <Q as TokenQuantity>::LocalDeposit>;
