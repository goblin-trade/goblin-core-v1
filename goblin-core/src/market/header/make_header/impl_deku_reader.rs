use deku::DekuError;
use deku::DekuReader;
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;

use crate::axis::occupancy::OccupancyEnum;
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};
use crate::quantities::{BaseLots, InnerPos};

use super::MakeHeader;

impl<'a> DekuReader<'a, ()> for MakeHeader {
    #[inline]
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        _ctx: (),
    ) -> Result<Self, DekuError> {
        let lane = read_lane::<R, 5>(reader)?;

        Ok(Self {
            inner_pos: InnerPos::from_raw(unpack(lane, 0, 8)),
            occupancy_enum: OccupancyEnum::from_raw(unpack(lane, 8, 1)),
            inner_enum_raw: unpack_bool(lane, 9),
            base_lots_u32: BaseLots::from_raw(unpack(lane, 10, 30)),
        })
    }
}
