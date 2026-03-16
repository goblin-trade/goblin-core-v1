use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::generic_iterator::{
        outer_bitmap_index::{market_item::MarketItemV2, outer_bitmap_item::OuterBitmapItemV2},
        GenericIterator,
    },
};

pub type OuterBitmapIndexIteratorV2<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
= GenericIterator<
    MarketItemV2<'a, M, B, Q>,
    In::OuterBitmapIndexIter,
    OuterBitmapItemV2<M, B, Q, In>,
>;
