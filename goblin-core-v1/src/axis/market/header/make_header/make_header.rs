use crate::{
    instructions::make_variant::MakeVariant,
    quantities::{BaseLots, InnerPos},
};

pub struct MakeHeader {
    pub inner_pos: InnerPos,
    pub base_lots: BaseLots,
    pub make_variant: MakeVariant,
}
