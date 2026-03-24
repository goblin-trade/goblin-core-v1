use crate::{
    axis::{
        leg::{leg_coordinates::LegCoordinates, leg_matcher::LegMatcher, Base, LegEnum, Quote},
        market::{header::update_header::UpdateHeader, market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::{update_marker::UpdateMarker, UpdateEnum},
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::get_leg_in::get_leg_in,
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, FullCoordinates,
    },
    quantities::{BaseLots, Ticks},
    settlement::local_delta::LocalDelta,
    state::{
        bitmap::{
            inner_bitmap::{preimage::InnerBitmapPreimage, InnerBitmap},
            outer_bitmap::{
                active_outer_bitmap::ActiveOuterBitmap, outer_bitmap_state::OuterBitmapState,
                preimage::OuterBitmapPreimage,
            },
        },
        resting_order::preimage::RestingOrderPreimage,
        MarketState, Preimage, SlotKey,
    },
    types::StoreReader,
};

pub fn update_inner<M, B, Q, In, U>(
    ctx: &DecodeCtx,
    local_delta: &mut LocalDelta,
    MarketAndKey { market, market_key }: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    full_coordinates: FullCoordinates,
    base_lots: BaseLots,
    outer_bitmap_key: &SlotKey<OuterBitmapPreimage<M, B, Q>>,
    active_outer_bitmap: &ActiveOuterBitmap,
    inner_bitmap_key: &SlotKey<InnerBitmapPreimage<M, B, Q>>,
    inner_bitmap_state: &InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
    U: UpdateMarker,
{
    // Check if active. If inactive, we cannot increase or decrease
    // Update slot
    let resting_order_key = RestingOrderPreimage {
        inner_bitmap_key: *inner_bitmap_key,
        inner_pos: full_coordinates.inner_pos,
    }
    .hash();

    let resting_order_state = resting_order_key.load();

    let base_lot_size = Base::get(&market.lot_size_pair);
    let price = Ticks::from(full_coordinates);
    let delta = In::maker_deposit(base_lots, base_lot_size, market.tick_size, price);
    Ok(())
}
