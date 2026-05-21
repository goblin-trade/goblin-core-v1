use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, InnerPos, Position, INNER_POS, POS_1},
    require,
    settlement::local_delta::LocalDelta,
    state::{bitmap::Bitmap, MarketState},
    types::Address,
};

pub fn ix_limit<M, B, Q>(
    msg_sender: &Address,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    position: Position,
    region: MakeRegion,
    inner_bitmap_state: &mut Bitmap<POS_1, INNER_POS>,
    base_lots: BaseLots,
    leg_enum: LegEnum,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    Ok(())
}
