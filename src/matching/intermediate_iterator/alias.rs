use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::intermediate_iterator::IntermediateIterator,
    state::bitmap::{
        inner_bitmap::InnerBitmap, outer_bitmap::active_outer_bitmap::ActiveOuterBitmap,
    },
};

pub type IntermediateOuterPosIter<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
= IntermediateIterator<In::OuterPosIter, ActiveOuterBitmap<M, B, Q>>;

pub type IntermediateInnerPosIter<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
= IntermediateIterator<In::InnerPosIter, InnerBitmap<M, B, Q>>;
