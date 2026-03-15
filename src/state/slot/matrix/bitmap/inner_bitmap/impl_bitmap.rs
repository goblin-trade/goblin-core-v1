use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::row_column::RowColumn,
    state::bitmap::{inner_bitmap::InnerBitmap, Bitmap},
};

impl<M, B, Q, In> Bitmap<RowColumn<In>> for InnerBitmap<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    fn active(&self, pos: RowColumn<In>) -> bool {
        let row_bits = self.inner[pos.row.inner as usize];
        let mask = 1u8 << pos.column.inner;
        (row_bits & mask) != 0
    }
}
