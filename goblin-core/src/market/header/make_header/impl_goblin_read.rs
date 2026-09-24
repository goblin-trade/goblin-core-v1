use crate::codec::{CodecResult, GoblinRead, Reader};

use crate::axis::occupancy::OccupancyEnum;
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};
use crate::quantities::{BaseLots, InnerPos};

use super::MakeHeader;

impl<'de> GoblinRead<'de, ()> for MakeHeader {
    #[inline]
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let lane = read_lane::<5>(reader)?;

        Ok(Self {
            inner_pos: InnerPos::from_raw(unpack(lane, 0, 8)),
            occupancy_enum: OccupancyEnum::from_raw(unpack(lane, 8, 1)),
            inner_enum_raw: unpack_bool(lane, 9),
            base_lots_u32: BaseLots::from_raw(unpack(lane, 10, 30)),
        })
    }
}
