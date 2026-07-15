use crate::axis::{leg::Pair, token::token_marker::TokenMarker};

pub type TokenAddressPair<B, Q> =
    Pair<<B as TokenMarker>::TokenAddress, <Q as TokenMarker>::TokenAddress>;
