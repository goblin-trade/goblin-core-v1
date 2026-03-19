use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::{column::Column, inner_pos::InnerPos, row::Row},
    state::bitmap::{inner_bitmap::InnerBitmap, Bitmap},
};

impl<M, B, Q, In> Bitmap<InnerPos<In>> for InnerBitmap<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    fn active(&self, pos: InnerPos<In>) -> bool {
        let row = Row::from(pos);
        let column = Column::from(pos);

        let row_bits = self.inner[row.inner as usize];
        let mask = 1u8 << column.inner;
        (row_bits & mask) != 0
    }
}
