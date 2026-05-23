use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::PosHeader,
    state::bitmap::alias::InnerBitmap,
};

pub trait UpdateMarker {
    fn process_update<'a, M, B, Q, In>(
        readables: &Readables<M, B, Q>,
        pos_header: PosHeader,
        writables: &mut Writables,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;
}
