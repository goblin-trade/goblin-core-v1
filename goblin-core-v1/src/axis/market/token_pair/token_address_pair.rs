use crate::axis::{leg::Pair, market::TokenPair, token::token_quantity::TokenQuantity};

pub type TokenAddressPair<TP> = Pair<
    <<TP as TokenPair>::Base as TokenQuantity>::TokenAddress,
    <<TP as TokenPair>::Quote as TokenQuantity>::TokenAddress,
>;
