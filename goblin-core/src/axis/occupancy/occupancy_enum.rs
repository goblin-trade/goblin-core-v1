use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;
use goblin_macros::define_axis;

#[define_axis]
#[derive(DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
#[deku(id_type = "u8", bits = 1, bit_order = "lsb")]
pub enum OccupancyEnum {
    Vacant = 0,
    Occupied = 1,
}
