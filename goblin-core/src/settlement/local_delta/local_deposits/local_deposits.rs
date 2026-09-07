use crate::{
    axis::{leg::Pair, token::token_quantity::TokenQuantity},
    axis_helpers::TokenPair,
};

/// Local deposits for a market
pub type LocalDeposits<TP> = Pair<
    <<TP as TokenPair>::Base as TokenQuantity>::LocalDeposit,
    <<TP as TokenPair>::Quote as TokenQuantity>::LocalDeposit,
>;
