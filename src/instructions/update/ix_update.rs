use crate::{
    axis::{
        leg::{leg_coordinates::LegCoordinates, Base, Quote},
        market::{header::update_header::UpdateHeader, market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::get_leg_in::get_leg_in,
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, FullCoordinates,
    },
    quantities::Ticks,
    settlement::local_delta::LocalDelta,
    state::{
        bitmap::{
            inner_bitmap::{preimage::InnerBitmapPreimage, InnerBitmap},
            outer_bitmap::{outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage},
        },
        MarketState, SlotKey,
    },
    types::StoreReader,
};

pub fn ix_update<M, B, Q>(
    ctx: &DecodeCtx,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    outer_bitmap_index: OuterBitmapIndex,
    outer_bitmap_key: &SlotKey<OuterBitmapPreimage<M, B, Q>>,
    outer_bitmap_state: &OuterBitmapState,
    outer_pos: OuterPos,
    inner_bitmap_key: &SlotKey<InnerBitmapPreimage<M, B, Q>>,
    inner_bitmap_state: &InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    let header = UpdateHeader::try_decode(ctx)?;

    let coordinate = FullCoordinates {
        outer_bitmap_index,
        outer_pos,
        inner_pos: header.inner_pos,
    };
    let price = Ticks::from(coordinate);

    let leg_in = get_leg_in(price, &market_state.last_coordinates)?;

    Ok(())
}
