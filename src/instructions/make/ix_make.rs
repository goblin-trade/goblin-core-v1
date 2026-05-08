use crate::{
    axis::{
        market::{header::make_header::MakeHeader, market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::{make_variant::MakeVariant, open::ix_open, update::ix_update},
    matching::region::make_region::MakeRegion,
    quantities::{Position, INNER_POS},
    settlement::local_delta::LocalDelta,
    state::{bitmap::Bitmap, MarketState},
    types::Address,
};

pub fn ix_make<M, B, Q>(
    msg_sender: &Address,
    ctx: &DecodeCtx,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    position_1: Position,
    inner_bitmap_state: &mut Bitmap<INNER_POS>,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    let MakeHeader {
        inner_pos,
        base_lots,
        make_variant,
    } = MakeHeader::try_decode(ctx)?;

    let position_2 = position_1 + Position::from(inner_pos);
    let region_2 = MakeRegion::new(&market_state.last_positions, position_2);

    match make_variant {
        MakeVariant::Update(update_enum) => ix_update::<M, B, Q>(
            msg_sender,
            local_delta,
            market_and_key,
            position_2,
            region_2,
            inner_bitmap_state,
            base_lots,
            update_enum,
        ),
        MakeVariant::Open(leg_enum) => ix_open::<M, B, Q>(
            msg_sender,
            local_delta,
            market_and_key,
            market_state,
            position_2,
            region_2,
            inner_bitmap_state,
            base_lots,
            leg_enum,
        ),
    }
}
