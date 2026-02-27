use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        active_iterator::resting_order::quote_pair::QuotePair,
        bitmap::{
            outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row_column::RowColumn,
            PriceCoordinates,
        },
    },
    quantities::{QuoteLotsPerBaseUnitPerTick, Ticks},
    state::{
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
};

pub struct RestingOrderItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub outer_bitmap_index: OuterBitmapIndex<In>,
    pub outer_pos: OuterPos<In>,
    pub row_column: RowColumn<In>,
    pub resting_order_key: SlotKey<RestingOrderPreimage<M, B, Q, In>>,
    pub resting_order: RestingOrder<M, B, Q>,
    pub limit_reached: bool,
}

impl<M, B, Q, In> RestingOrderItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn price(&self) -> Ticks {
        PriceCoordinates {
            outer_bitmap_index: self.outer_bitmap_index,
            outer_pos: self.outer_pos,
            row: self.row_column.row,
        }
        .into()
    }

    pub fn quote(&self, tick_size: QuoteLotsPerBaseUnitPerTick) -> QuotePair<In> {
        let price = self.price();

        QuotePair {
            quote: In::matching_lots_maker(self.resting_order.size, tick_size, price),
            quote_opposite: In::Opposite::matching_lots_maker(
                self.resting_order.size,
                tick_size,
                price,
            ),
        }
    }
}
