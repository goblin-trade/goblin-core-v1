use crate::axis::{
    leg::Pair,
    token::{token_deltas::TokenDeltas, token_index::TokenIndex},
};

pub type TokenAddressPair<B, Q> = Pair<
    <<B as TokenDeltas>::TokenIndex as TokenIndex>::TokenAddress,
    <<Q as TokenDeltas>::TokenIndex as TokenIndex>::TokenAddress,
>;
