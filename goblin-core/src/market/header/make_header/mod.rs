use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::{
    axis::occupancy::OccupancyEnum,
    quantities::{BaseLots, InnerPos},
};

/// Make instruction header.
///
/// The wire layout is 5 bytes. base_lots_u32` is shifted 2 bits to fit
/// in `occupancy_enum` and `inner_enum_raw`
///
#[derive(DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct MakeHeader {
    pub inner_pos: InnerPos,

    pub occupancy_enum: OccupancyEnum,

    #[deku(bits = "1")]
    pub inner_enum_raw: bool,

    #[deku(bits = "30", bit_order = "lsb")]
    pub base_lots_u32: BaseLots<u32>,
}
