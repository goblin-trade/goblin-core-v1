use crate::{
    axis::{
        leg::{Base, Quote},
        token::token_marker::TokenMarker,
    },
    market::TokenPair,
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
