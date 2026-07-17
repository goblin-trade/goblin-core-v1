use crate::axis::{leg::Pair, token::token_quantity::TokenQuantity};

pub type TokenAddressPair<B, Q> =
    Pair<<B as TokenQuantity>::TokenAddress, <Q as TokenQuantity>::TokenAddress>;
