use crate::{
    axis::{
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    instructions::update::process_update_cases,
    matching::region::make_region::MakeRegion,
    quantities::{BaseLots, Position, INNER_POS},
    require,
    settlement::local_delta::LocalDelta,
    state::bitmap::Bitmap,
    types::Address,
};

pub fn ix_update<M, B, Q>(
    msg_sender: &Address,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    position_2: Position,
    region_2: MakeRegion,
    inner_bitmap_state: &mut Bitmap<INNER_POS>,
    base_lots: BaseLots,
    update_enum: UpdateEnum,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    // Cannot update in `Spread` region as it has no orders
    let MakeRegion::In(leg_in) = region_2 else {
        return Err(GoblinError::NoRestingOrder);
    };

    require!(
        inner_bitmap_state.index_active(position_2.into()),
        GoblinError::NoRestingOrder
    );

    process_update_cases::<M, B, Q>(
        msg_sender,
        &mut local_delta.local_sender_delta,
        market_and_key,
        position_2,
        inner_bitmap_state,
        base_lots,
        update_enum,
        leg_in,
    )
}
