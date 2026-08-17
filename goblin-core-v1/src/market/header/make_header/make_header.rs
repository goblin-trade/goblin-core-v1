use crate::{
    axis::occupancy::OccupancyEnum,
    quantities::{BaseLots, InnerPos},
};

pub struct MakeHeader {
    pub inner_pos: InnerPos,
    pub occupancy_enum: OccupancyEnum,
    pub inner_enum_raw: bool,
    pub base_lots: BaseLots,
}
