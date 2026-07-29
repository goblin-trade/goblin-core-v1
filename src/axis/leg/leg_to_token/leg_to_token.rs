use crate::axis::{
    leg::{Base, Quote},
    market::token_pair::TokenPair,
    token::token_marker::TokenMarker,
};

/// Maps a leg marker + a token pair to the token marker on that side.
pub trait LegToToken<TP: TokenPair> {
    type Selected: TokenMarker;
}

impl<TP: TokenPair> LegToToken<TP> for Base {
    type Selected = TP::Base;
}

impl<TP: TokenPair> LegToToken<TP> for Quote {
    type Selected = TP::Quote;
}
