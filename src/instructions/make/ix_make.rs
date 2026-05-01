use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{header::make_header::MakeHeader, market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::{make_variant::MakeVariant, open::ix_open, update::ix_update},
    quantities::{Position, INNER_POS_V2},
    settlement::local_delta::LocalDelta,
    state::{
        bitmap_v2::{preimage::BitmapPreimageV2, BitmapV2},
        MarketState, SlotKey,
    },
};

pub fn ix_make<M, B, Q>(
    ctx: &DecodeCtx,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    position_1: Position,
    inner_bitmap_key: &SlotKey<BitmapPreimageV2<M, B, Q, INNER_POS_V2>>,
    inner_bitmap_state: &mut BitmapV2<INNER_POS_V2>,
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

    match make_variant {
        MakeVariant::Update(update_enum) => ix_update::<M, B, Q>(
            local_delta,
            market_and_key,
            market_state,
            position_2,
            &inner_bitmap_key,
            inner_bitmap_state,
            base_lots,
            update_enum,
        )?,
        MakeVariant::Open(leg_enum) => match leg_enum {
            LegEnum::Base => ix_open::<M, B, Q, Base>(
                local_delta,
                market_and_key,
                market_state,
                position_2,
                inner_bitmap_key,
                inner_bitmap_state,
                base_lots,
            )?,
            LegEnum::Quote => ix_open::<M, B, Q, Quote>(
                local_delta,
                market_and_key,
                market_state,
                position_2,
                inner_bitmap_key,
                inner_bitmap_state,
                base_lots,
            )?,
        },
    }

    Ok(())
}
