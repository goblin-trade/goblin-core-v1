//
// There are 2 ways to define it
// 1. Local- implemented on ActiveOuterBitmap. Just return value that is active.
// No need to check limit, as limit is global (based on price)
//
// 2. Wide- wraps ActiveOuterBitmapIterator. The result will be
// (OuterBitmapIndex, InnerBitmapIndex, InnerBitmap)
// This looks more logical.
// OuterBitmap -> InnerBitmap -> RestingOrder (coordinate)

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::active_iterator::outer_bitmap::ActiveOuterBitmapIterator,
};

pub struct ActiveInnerBitmapIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub outer_iterator: ActiveOuterBitmapIterator<'a, M, B, Q, In>,
}
