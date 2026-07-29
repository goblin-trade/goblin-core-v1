use crate::axis::{leg::Pair, token::token_marker::TokenMarker};

pub trait TokenPair {
    type Base: TokenMarker;
    type Quote: TokenMarker;
}

impl<B, Q> TokenPair for Pair<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    type Base = B;
    type Quote = Q;
}
