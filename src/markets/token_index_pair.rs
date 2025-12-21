use crate::{markets::MarketVariant, token::TokenMarker, types::Pair};

pub type TokenIndexPair<M: MarketVariant, B: TokenMarker, Q: TokenMarker> =
    Pair<B::TokenIndex<M>, Q::TokenIndex<M>>;

// TODO common Decodable implementation
