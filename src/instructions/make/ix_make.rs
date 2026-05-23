use crate::{
    axis::{
        market::{
            header::make_header::MakeHeader, market_marker::MarketMarker, Readables, Writables,
        },
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::{
        limit::ix_limit, make_variant::MakeVariant, open::ix_open::ix_open,
        update::ix_update::ix_update, PosHeader,
    },
    quantities::{Pos2, SafePosition, POS_1},
    state::bitmap::alias::InnerBitmap,
};

pub fn ix_make<M, B, Q>(
    ctx: &DecodeCtx,
    readables: &Readables<M, B, Q>,
    pos_1: SafePosition<POS_1>,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
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

    let pos_2 = Pos2::new(pos_1, inner_pos);
    let position = pos_2.into();

    let pos_header = PosHeader {
        position,
        base_lots,
    };

    match make_variant {
        MakeVariant::Update(update_enum) => ix_update(
            readables,
            pos_header,
            update_enum,
            writables,
            inner_bitmap_state,
        ),
        MakeVariant::Open(leg_enum) => ix_open(
            readables,
            pos_header,
            leg_enum,
            writables,
            inner_bitmap_state,
        ),
        MakeVariant::Limit(leg_enum) => ix_limit(
            readables,
            pos_header,
            leg_enum,
            writables,
            inner_bitmap_state,
        ),
    }
}
