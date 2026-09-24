mod impl_deku_reader;

#[cfg(feature = "encode")]
mod impl_deku_writer;

use crate::{
    axis::occupancy::OccupancyEnum,
    quantities::{BaseLots, InnerPos},
};

/// Make instruction header.
///
/// The wire layout is 5 bytes. `base_lots_u32` is shifted 2 bits to fit
/// in `occupancy_enum` and `inner_enum_raw`. deku's `bits` feature is off, so
/// the five bytes are decoded as a single LSB-first lane.
pub struct MakeHeader {
    pub inner_pos: InnerPos,

    pub occupancy_enum: OccupancyEnum,

    pub inner_enum_raw: bool,

    pub base_lots_u32: BaseLots<u32>,
}
