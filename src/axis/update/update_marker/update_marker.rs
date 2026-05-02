use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, Position, INNER_POS_V2},
    settlement::local_delta::LocalSenderDelta,
    state::bitmap_v2::BitmapV2,
};

pub trait UpdateMarker {
    fn process_update<M, B, Q, In>(
        local_sender_delta: &mut LocalSenderDelta,
        market_and_key: &MarketAndKey<M, B, Q>,
        position_2: Position,
        base_lots: BaseLots,
        inner_bitmap_state: &mut BitmapV2<INNER_POS_V2>,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;
}
