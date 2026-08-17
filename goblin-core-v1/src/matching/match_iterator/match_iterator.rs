use crate::{
    axis::leg::leg_matcher::LegMatcher,
    axis_helpers::MarketSpec,
    quantities::{Position, INNER_POS, POS_1},
    state::{
        bitmap::{bitmap_reader::BitmapReader, Bitmap},
        resting_order::preimage::RestingOrderPreimage,
        KeyValue, MarketPreimage, Preimage, SlotKey,
    },
};

pub struct RestingOrderEntry<MS: MarketSpec> {
    pub position: Position,
    pub resting_order_key_value: KeyValue<RestingOrderPreimage<MS>>,
}

pub fn match_iterator<MS: MarketSpec, In: LegMatcher>(
    market_key: SlotKey<MarketPreimage<MS>>,
    last_position: Position,
    limit: Position,
) -> impl Iterator<Item = RestingOrderEntry<MS>> {
    let range = In::get_range(last_position, limit);
    Bitmap::<POS_1, INNER_POS>::active_iterator::<MS, In>(market_key, range).map(move |pos_2| {
        let position = pos_2.into();

        let resting_order_key_value = RestingOrderPreimage::<MS> {
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
