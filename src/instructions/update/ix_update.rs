use crate::{
    axis::{
        leg::{leg_coordinates::LegCoordinates, Base, LegEnum, Quote},
        market::{header::update_header::UpdateHeader, market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::{Increase, UpdateEnum},
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::{get_leg_in::get_leg_in, update_inner::update_inner},
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, FullCoordinates,
    },
    quantities::Ticks,
    require,
    settlement::local_delta::LocalDelta,
    state::{
        bitmap::{
            inner_bitmap::{preimage::InnerBitmapPreimage, InnerBitmap},
            outer_bitmap::{
                active_outer_bitmap::ActiveOuterBitmap, outer_bitmap_state::OuterBitmapState,
                preimage::OuterBitmapPreimage,
            },
            Bitmap,
        },
        MarketState, SlotKey,
    },
    types::StoreReader,
};

pub fn ix_update<M, B, Q>(
    ctx: &DecodeCtx,
    local_delta: &mut LocalDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    market_state: &mut MarketState,
    outer_bitmap_index: OuterBitmapIndex,
    outer_bitmap_key: &SlotKey<OuterBitmapPreimage<M, B, Q>>,
    active_outer_bitmap: &ActiveOuterBitmap,
    outer_pos: OuterPos,
    inner_bitmap_key: &SlotKey<InnerBitmapPreimage<M, B, Q>>,
    inner_bitmap_state: &InnerBitmap,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    let header = UpdateHeader::try_decode(ctx)?;
    require!(
        inner_bitmap_state.active(header.inner_pos),
        GoblinError::NoRestingOrder
    );

    let full_coordinates = FullCoordinates {
        outer_bitmap_index,
        outer_pos,
        inner_pos: header.inner_pos,
    };
    let price = Ticks::from(full_coordinates);

    let leg_in = get_leg_in(price, &market_state.last_coordinates)?;

    match (leg_in, header.update_variant) {
        (LegEnum::Base, UpdateEnum::Increase) => update_inner::<M, B, Q, Base, Increase>(
            ctx,
            local_delta,
            market_and_key,
            market_state,
            full_coordinates,
            header.base_lots,
            outer_bitmap_key,
            active_outer_bitmap,
            inner_bitmap_key,
            inner_bitmap_state,
        )?,
        (LegEnum::Base, UpdateEnum::Decrease) => {}
        (LegEnum::Quote, UpdateEnum::Increase) => {}
        (LegEnum::Quote, UpdateEnum::Decrease) => {}
    }

    Ok(())
}
