use crate::axis::{
    leg::Pair,
    token::{token_index::TokenIndex, token_marker::TokenMarker},
};

pub type TokenAddressPair<B, Q> = Pair<
    <<B as TokenMarker>::TokenIndex as TokenIndex<B>>::TokenAddress,
    <<Q as TokenMarker>::TokenIndex as TokenIndex<Q>>::TokenAddress,
>;
