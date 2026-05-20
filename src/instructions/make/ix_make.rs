use crate::{
    axis::{
        market::{header::make_header::MakeHeader, market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::{make_variant::MakeVariant, open::ix_open, update::ix_update},
    matching::region::make_region::MakeRegion,
    quantities::{SafePosition, INNER_POS, POS_1, POS_2},
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
    pos_1: SafePosition<POS_1>,
    inner_bitmap_state: &mut Bitmap<POS_1, INNER_POS>,
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

    let pos_2 = SafePosition::<POS_2>::new(pos_1, inner_pos);
    let position = pos_2.position();
    let region = MakeRegion::new(&market_state.last_positions, position);

    match make_variant {
        MakeVariant::Update(update_enum) => ix_update::<M, B, Q>(
            msg_sender,
            local_delta,
            market_and_key,
            position,
            region,
            inner_bitmap_state,
            base_lots,
            update_enum,
        ),
        MakeVariant::Open(leg_enum) => ix_open::<M, B, Q>(
            msg_sender,
            local_delta,
            market_and_key,
            market_state,
            position,
            region,
            inner_bitmap_state,
            base_lots,
            leg_enum,
        ),
    }
}
