use crate::{
    axis::LegMatcher,
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    quantities::{Position, INNER_POS, POS_1},
    require,
    state::{
        bitmap::{Bitmap, BitmapReader},
        KeyValue, MarketPreimage, MarketState, Preimage, RestingOrderPreimage, SlotKey,
    },
};

pub struct RestingOrderEntry<TP: TokenPair> {
    pub position: Position,
    pub resting_order_key_value: KeyValue<RestingOrderPreimage<TP>>,
}

pub fn match_iterator<'a, TP: TokenPair, In: LegMatcher + 'a>(
    market_key: SlotKey<MarketPreimage<TP>>,
    limit: Position,
    market_state: &'a mut MarketState,
) -> Result<impl Iterator<Item = RestingOrderEntry<TP>> + 'a, GoblinError> {
    let last_position = In::get_leg_mut(&mut market_state.last_positions);

    require!(
        In::in_region(*last_position, limit),
        GoblinError::TakerPriceLimitReached
    );

    let range = In::get_range(*last_position, limit);
    Ok(
        Bitmap::<POS_1, INNER_POS>::active_iterator::<TP, In>(market_key, range).map(
            move |pos_2| {
                let position = pos_2.into();
                *last_position = position;

                let resting_order_key_value = RestingOrderPreimage::<TP> {
                    market_key,
                    position,
                }
                .key_value();

                RestingOrderEntry {
                    position,
                    resting_order_key_value,
                }
            },
        ),
    )
}
