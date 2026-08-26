use crate::{
    axis::{leg::Pair, token::token_quantity::TokenQuantity},
    axis_helpers::TokenPair,
};

pub type TokenIndexPair<TP> = Pair<
    <<TP as TokenPair>::Base as TokenQuantity>::TokenIndex,
    <<TP as TokenPair>::Quote as TokenQuantity>::TokenIndex,
>;
