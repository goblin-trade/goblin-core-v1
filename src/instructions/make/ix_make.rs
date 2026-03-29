use crate::{
    axis::{
        market::{
            header::{make_header::MakeHeader, update_header::UpdateHeader},
            market_marker::MarketMarker,
            MarketAndKey,
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::process_update_cases::process_update_cases,
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, FullCoordinates,
    },
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

pub fn ix_make<M, B, Q>(
    ctx: &DecodeCtx,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    outer_bitmap_index: OuterBitmapIndex,
    outer_pos: OuterPos,
    inner_bitmap_key: &SlotKey<InnerBitmapPreimage<M, B, Q>>,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    let header = MakeHeader::try_decode(ctx)?;

    Ok(())
}
