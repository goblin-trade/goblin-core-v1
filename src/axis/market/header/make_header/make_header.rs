use crate::{
    instructions::make_variant::MakeVariant,
    quantities::{BaseLots, InnerPosV2},
};

pub struct MakeHeader {
    pub inner_pos: InnerPosV2,
    pub base_lots: BaseLots,
    pub make_variant: MakeVariant,
}
