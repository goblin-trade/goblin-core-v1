use std::ops::RangeInclusive;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
    },
    state::{
        bitmap::outer_bitmap::{
            outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage,
        },
        MarketPreimage, Preimage, SlotKey,
    },
};

pub fn match_order<'a, M, B, Q, In>(
    market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,
    range: RangeInclusive<(OuterBitmapIndex<In>, OuterPos<In>, InnerPos<In>)>,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let outer_bitmap_iter = In::outer_bitmap_index_iter(range.start().0..=range.end().0);

    let final_iter = outer_bitmap_iter.filter_map(move |outer_bitmap_index| {
        let preimage = OuterBitmapPreimage {
            market_key: *market_key,
            outer_bitmap_index,
        };
        let outer_bitmap_key = preimage.hash();
        let outer_bitmap = outer_bitmap_key.load();
        let outer_bitmap_state = OuterBitmapState::from(outer_bitmap);

        match outer_bitmap_state {
            OuterBitmapState::Active(_) => Some((outer_bitmap_index, outer_bitmap_state)),
            _ => None,
        }
    });

    Ok(())
}
