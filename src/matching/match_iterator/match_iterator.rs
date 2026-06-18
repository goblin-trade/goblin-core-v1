use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_reader::TokenReader,
    },
    quantities::{Position, INNER_POS, POS_1},
    state::{
        bitmap::{bitmap_reader::BitmapReader, Bitmap},
        resting_order::preimage::RestingOrderPreimage,
        KeyValue, MarketPreimage, Preimage, SlotKey,
    },
};

pub struct RestingOrderEntry<M, B, Q>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
{
    pub position: Position,
    pub resting_order_key_value: KeyValue<RestingOrderPreimage<M, B, Q>>,
}

pub fn match_iterator<M, B, Q, In>(
    market_key: SlotKey<MarketPreimage<M, B, Q>>,
    last_position: Position,
    limit: Position,
) -> impl Iterator<Item = RestingOrderEntry<M, B, Q>>
where
    M: MarketMarker,
    B: TokenReader,
    Q: TokenReader,
    In: LegMatcher,
{
    let range = In::get_range(last_position, limit);
    Bitmap::<POS_1, INNER_POS>::active_iterator::<M, B, Q, In>(market_key, range).map(
        move |pos_2| {
            let position = pos_2.into();

            let resting_order_key_value = RestingOrderPreimage::<M, B, Q> {
                market_key,
                position,
            }
            .key_value();

            RestingOrderEntry {
                position,
                resting_order_key_value,
            }
        },
    )
}
