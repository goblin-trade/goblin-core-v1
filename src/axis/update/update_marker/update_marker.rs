use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, CommonMarket},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, Position, INNER_POS_V2},
    settlement::local_delta::LocalSenderDelta,
    state::{bitmap_v2::BitmapV2, resting_order::preimage::RestingOrderPreimage, SlotKey},
};

pub trait UpdateMarker {
    fn process_update<M, B, Q, In>(
        local_sender_delta: &mut LocalSenderDelta,
        market: &CommonMarket<M, B, Q>,
        resting_order_key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
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
