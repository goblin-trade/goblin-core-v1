use crate::{
    axis::market::{
        header::make_header::MakeHeader, market_spec::MarketSpec, Readables, Writables,
    },
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    instructions::{
        make_variant::MakeVariant, open::ix_open::ix_open, update::ix_update::ix_update,
        MakeReadables, PosHeader,
    },
    quantities::{Pos2, SafePosition, POS_1},
    state::bitmap::alias::InnerBitmap,
};

pub fn ix_make<MS: MarketSpec>(
    ctx: &DecodeCtx,
    readables: &Readables<MS>,
    pos_1: SafePosition<POS_1>,
    writables: &mut Writables,
    inner_bitmap_state: &mut InnerBitmap,
) -> Result<(), GoblinError> {
    let MakeHeader {
        inner_pos,
        base_lots,
        make_variant,
    } = MakeHeader::try_fixed_decode(ctx)?;

    let pos_2 = Pos2::new(pos_1, inner_pos);
    let position = pos_2.into();

    let make_readables = &MakeReadables {
        readables,
        pos_header: PosHeader {
            position,
            base_lots,
        },
    };

    // TODO use for_axes! and generic
    match make_variant {
        MakeVariant::Update(update_enum) => {
            ix_update(make_readables, update_enum, writables, inner_bitmap_state)
        }
        MakeVariant::Open(leg_enum) => {
            ix_open(make_readables, leg_enum, writables, inner_bitmap_state)
        }
    }
}
