use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, LegEnum},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, Position, INNER_POS_V2},
    require,
    settlement::local_delta::LocalDelta,
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        MarketState, Preimage, SlotKey,
    },
    types::Address,
};

pub fn process_open<M, B, Q, In>(
    msg_sender: &Address,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    position_2: Position,
    region_2: MakeRegion,
    inner_bitmap_key: &SlotKey<BitmapPreimageV2<M, B, Q, INNER_POS_V2>>,
    inner_bitmap_state: &mut BitmapV2<INNER_POS_V2>,
    base_lots: BaseLots,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let resting_order_key = RestingOrderPreimage {
        market_key: market_and_key.market_key,
        position: position_2,
    }
    .hash();

    let mut resting_order = RestingOrder {
        maker: *msg_sender,
        base_lots,
    };

    Ok(())
}
