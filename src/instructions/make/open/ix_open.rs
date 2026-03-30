use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, LegEnum},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    matching::bitmap::FullCoordinates,
    quantities::BaseLots,
    require,
    settlement::local_delta::LocalDelta,
    state::{
        bitmap::{
            inner_bitmap::{preimage::InnerBitmapPreimage, InnerBitmap},
            Bitmap,
        },
        resting_order::preimage::RestingOrderPreimage,
        MarketState, Preimage, SlotKey,
    },
};

pub fn ix_open<M, B, Q, In>(
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    full_coordinates: &FullCoordinates,
    inner_bitmap_key: &SlotKey<InnerBitmapPreimage<M, B, Q>>,
    inner_bitmap_state: &mut InnerBitmap,
    base_lots: BaseLots,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    require!(
        In::valid_open_price(market_state, (*full_coordinates).into()),
        GoblinError::InvalidOpenPrice
    );
    Ok(())
}
