use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{market_marker::MarketMarker, MarketAndKey, Readables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::{MakeWritables, PosHeader},
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, InnerPos, Position, INNER_POS, POS_1},
    require,
    settlement::local_delta::LocalDelta,
    state::{bitmap::Bitmap, MarketState},
    types::Address,
};

impl<'a> MakeWritables<'a> {
    pub fn ix_limit<M, B, Q>(
        &mut self,
        readables: &Readables<M, B, Q>,
        pos_header: PosHeader,
        leg_enum: LegEnum,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        Ok(())
    }
}
