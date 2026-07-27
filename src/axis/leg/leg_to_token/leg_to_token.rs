use crate::axis::{
    leg::{Base, Quote},
    token::token_marker::TokenMarker,
};

/// Maps a leg marker + a (B, Q) pair to the token marker on that side.
pub trait LegToToken<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
{
    type Selected: TokenMarker;
}

impl<B, Q> LegToToken<B, Q> for Base
where
    B: TokenMarker,
    Q: TokenMarker,
{
    type Selected = B;
}

impl<B, Q> LegToToken<B, Q> for Quote
where
    B: TokenMarker,
    Q: TokenMarker,
{
    type Selected = Q;
}
