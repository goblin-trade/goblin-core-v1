use crate::{
    axis::leg::leg_matcher::LegMatcher,
    axis_helpers::TokenPair,
    quantities::{Position, INNER_POS, POS_1},
    state::{
        bitmap::{bitmap_reader::BitmapReader, Bitmap},
        resting_order::preimage::RestingOrderPreimage,
        KeyValue, MarketPreimage, Preimage, SlotKey,
    },
};

pub struct RestingOrderEntry<TP: TokenPair> {
    pub position: Position,
    pub resting_order_key_value: KeyValue<RestingOrderPreimage<TP>>,
}

pub fn match_iterator<TP: TokenPair, In: LegMatcher>(
    market_key: SlotKey<MarketPreimage<TP>>,
    last_position: Position,
    limit: Position,
) -> impl Iterator<Item = RestingOrderEntry<TP>> {
    let range = In::get_range(last_position, limit);
    Bitmap::<POS_1, INNER_POS>::active_iterator::<TP, In>(market_key, range).map(move |pos_2| {
        let position = pos_2.into();

        let resting_order_key_value = RestingOrderPreimage::<TP> {
            market_key,
            position,
        }
        .key_value();

        RestingOrderEntry {
            position,
            resting_order_key_value,
        }
    })
}
