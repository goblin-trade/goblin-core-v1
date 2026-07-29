use crate::axis::{leg::Pair, market::TokenPair, token::token_quantity::TokenQuantity};

pub type TokenIndexPair<TP> = Pair<
    <<TP as TokenPair>::Base as TokenQuantity>::TokenIndex,
    <<TP as TokenPair>::Quote as TokenQuantity>::TokenIndex,
>;
