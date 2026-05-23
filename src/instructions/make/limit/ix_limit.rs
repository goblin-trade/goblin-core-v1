use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, MarketAndKey, Readables, Writables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::PosHeader,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, InnerPos, Position, INNER_POS, POS_1},
    require,
    settlement::local_delta::LocalDelta,
    state::{
        bitmap::{alias::InnerBitmap, Bitmap},
        MarketState,
    },
    types::Address,
};

pub fn ix_limit<M, B, Q>(
    readables: &Readables<M, B, Q>,
    pos_header: PosHeader,
    leg_enum: LegEnum,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    Ok(())
}
