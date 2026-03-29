use crate::{
    instructions::make_variant::MakeVariant, matching::bitmap::inner_pos::InnerPos,
    quantities::BaseLots,
};

pub struct MakeHeader {
    pub inner_pos: InnerPos,
    pub base_lots: BaseLots,
    pub make_variant: MakeVariant,
}
