use crate::{
    axis::{
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    instructions::update::process_update_cases,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, Position, INNER_POS, POS_1},
    require,
    settlement::local_delta::LocalDelta,
    state::bitmap::Bitmap,
    types::Address,
};

pub fn ix_update<M, B, Q>(
    msg_sender: &Address,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    position: Position,
    region: MakeRegion,
    inner_bitmap_state: &mut Bitmap<POS_1, INNER_POS>,
    base_lots: BaseLots,
    update_enum: UpdateEnum,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    // Cannot update in `Spread` region as it has no orders
    let MakeRegion::In(leg_in) = region else {
        return Err(GoblinError::NoRestingOrder);
    };

    require!(
        inner_bitmap_state.index_active(position.into()),
        GoblinError::NoRestingOrder
    );

    process_update_cases::<M, B, Q>(
        msg_sender,
        &mut local_delta.local_sender_delta,
        market_and_key,
        position,
        inner_bitmap_state,
        base_lots,
        update_enum,
        leg_in,
    )
}
