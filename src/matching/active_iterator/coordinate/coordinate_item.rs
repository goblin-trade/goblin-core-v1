use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::FullCoordinates,
    state::{
        inner_bitmap::preimage::InnerBitmapPreimage,
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        SlotKey,
    },
};

pub struct CoordinateItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub full_coordinates: FullCoordinates<In>,
    pub inner_bitmap_key: SlotKey<InnerBitmapPreimage<M, B, Q, In>>,
    pub hash: SlotKey<RestingOrderPreimage<M, B, Q, In>>,
    pub resting_order: RestingOrder<M, B, Q>,
    // pub limit_reached: bool,
}

impl<M, B, Q, In> CoordinateItem<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub fn preimage(&self) -> RestingOrderPreimage<M, B, Q, In> {
        RestingOrderPreimage {
            inner_bitmap_key: self.inner_bitmap_key,
            inner_pos: self.full_coordinates.inner_pos,
        }
    }

    // pub fn price(&self) -> Ticks {
    //     PriceCoordinates {
    //         outer_bitmap_index: self.outer_bitmap_index,
    //         outer_pos: self.outer_pos,
    //         row: self.row_column.row,
    //     }
    //     .into()
    // }

    // pub fn quote(&self, tick_size: QuoteLotsPerBaseUnitPerTick) -> QuotePair<In> {
    //     let price = self.price();

    //     QuotePair {
    //         quote: In::matching_lots_maker(self.resting_order.size, tick_size, price),
    //         quote_opposite: In::Opposite::matching_lots_maker(
    //             self.resting_order.size,
    //             tick_size,
    //             price,
    //         ),
    //     }
    // }
}
