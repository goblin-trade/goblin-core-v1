use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, CommonMarket},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::bitmap::FullCoordinates,
    quantities::BaseLots,
    settlement::local_delta::LocalSenderDelta,
    state::{
        bitmap::inner_bitmap::InnerBitmap, resting_order::preimage::RestingOrderPreimage, SlotKey,
    },
};

pub trait UpdateMarker {
    fn process_update<M, B, Q, In>(
        local_sender_delta: &mut LocalSenderDelta,
        market: &CommonMarket<M, B, Q>,
        resting_order_key: &SlotKey<RestingOrderPreimage<M, B, Q>>,
        full_coordinates: &FullCoordinates,
        base_lots: BaseLots,
        inner_bitmap_state: &mut InnerBitmap,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;
}
