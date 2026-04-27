use crate::{
    axis::{
        leg::{Base, LegEnum, Quote},
        market::{header::make_header::MakeHeader, market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::{make_variant::MakeVariant, open::ix_open, update::ix_update},
    quantities::{OuterBitmapIndexV2, OuterPosV2},
    settlement::local_delta::LocalDelta,
    state::{MarketState, SlotKey},
};

pub fn ix_make<M, B, Q>(
    ctx: &DecodeCtx,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    outer_bitmap_index: OuterBitmapIndexV2,
    outer_pos: OuterPosV2,
    inner_bitmap_key: &SlotKey<InnerBitmapPreimage<M, B, Q>>,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    let header = MakeHeader::try_decode(ctx)?;

    // let full_coordinates = FullCoordinates {
    //     outer_bitmap_index,
    //     outer_pos,
    //     inner_pos: header.inner_pos,
    // };

    // match header.make_variant {
    //     MakeVariant::Update(update_enum) => ix_update::<M, B, Q>(
    //         local_delta,
    //         market_and_key,
    //         market_state,
    //         &full_coordinates,
    //         &inner_bitmap_key,
    //         inner_bitmap_state,
    //         header.base_lots,
    //         update_enum,
    //     )?,
    //     MakeVariant::Open(leg_enum) => match leg_enum {
    //         LegEnum::Base => ix_open::<M, B, Q, Base>(
    //             local_delta,
    //             market_and_key,
    //             market_state,
    //             &full_coordinates,
    //             inner_bitmap_key,
    //             inner_bitmap_state,
    //             header.base_lots,
    //         )?,
    //         LegEnum::Quote => ix_open::<M, B, Q, Quote>(
    //             local_delta,
    //             market_and_key,
    //             market_state,
    //             &full_coordinates,
    //             inner_bitmap_key,
    //             inner_bitmap_state,
    //             header.base_lots,
    //         )?,
    //     },
    // }

    Ok(())
}
