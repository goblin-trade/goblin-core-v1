use crate::{
    axis::{leg::Pair, token::token_quantity::TokenQuantity},
    market::TokenPair,
};

pub type TokenAddressPair<TP> = Pair<
    <<TP as TokenPair>::Base as TokenQuantity>::TokenAddress,
    <<TP as TokenPair>::Quote as TokenQuantity>::TokenAddress,
>;
