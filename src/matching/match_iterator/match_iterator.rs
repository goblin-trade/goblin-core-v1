use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{Pos2, INNER_POS, POS_1},
    state::{
        bitmap::{bitmap_reader::BitmapReader, Bitmap},
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        MarketPreimage, Preimage, SlotKey,
    },
};

pub struct RestingOrderEntry<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub position: Pos2,
    pub resting_order_key: SlotKey<RestingOrderPreimage<M, B, Q>>,
    pub resting_order: RestingOrder,
}

pub fn match_iterator<M, B, Q, In>(
    market_key: SlotKey<MarketPreimage<M, B, Q>>,
    last_position: Pos2,
    limit: Pos2,
) -> impl Iterator<Item = RestingOrderEntry<M, B, Q>>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let range = In::get_range(last_position, limit);
    Bitmap::<POS_1, INNER_POS>::active_iterator::<M, B, Q, In>(market_key, range).map(
        move |pos_2| {
            let preimage = RestingOrderPreimage::<M, B, Q> {
                market_key,
                position: pos_2,
            };
            let resting_order_key = preimage.hash();
            let resting_order = resting_order_key.load();

            RestingOrderEntry {
                position: pos_2,
                resting_order_key,
                resting_order,
            }
        },
    )
}
