use crate::{
    axis::update::UpdateEnum, matching::bitmap::inner_pos::InnerPos, quantities::BaseLots,
};

pub struct UpdateHeader {
    pub inner_pos: InnerPos,
    pub base_lots: BaseLots,
    pub update_variant: UpdateEnum,
}
