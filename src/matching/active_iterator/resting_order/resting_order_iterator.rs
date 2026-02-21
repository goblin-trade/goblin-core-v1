use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::{
        active_iterator::{
            inner_bitmap::ActiveInnerBitmapIterator,
            outer_bitmap::{outer_bitmap_item::OuterBitmapItem, ActiveOuterBitmapIterator},
        },
        bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, Coordinate},
    },
    state::{MarketPreimage, SlotKey},
};

pub struct RestingOrderIterator<'a, M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub active_inner_bitmap_iterator: ActiveInnerBitmapIterator<'a, M, B, Q, In>,

    /// The last returned outer bitmap item
    pub outer_bitmap_item: OuterBitmapItem<M, B, Q, In>,

    /// Begin lookup from this position
    pub outer_pos: Option<OuterPos<In>>,

    pub limit: OuterPos<In>,
}
