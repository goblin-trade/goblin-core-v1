use crate::axis::{leg::Pair, market::token_pair::TokenPair, token::token_quantity::TokenQuantity};

/// Deposit amounts read from calldata
pub type DepositPair<TP> = Pair<
    <<TP as TokenPair>::Base as TokenQuantity>::LocalDeposit,
    <<TP as TokenPair>::Quote as TokenQuantity>::LocalDeposit,
>;
