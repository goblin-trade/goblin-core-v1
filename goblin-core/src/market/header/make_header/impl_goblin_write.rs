use crate::codec::{CodecResult, GoblinWrite, Writer};

use crate::input_processor::bit_lane::{pack, pack_bool, write_lane};

use super::MakeHeader;

impl GoblinWrite<()> for MakeHeader {
    #[inline]
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        let mut lane = 0u64;
        lane = pack(lane, self.inner_pos.inner as u64, 0, 8);
        lane = pack(lane, self.occupancy_enum as u64, 8, 1);
        lane = pack_bool(lane, self.inner_enum_raw, 9);
        lane = pack(lane, self.base_lots_u32.inner as u64, 10, 30);

        write_lane::<5>(writer, lane)
    }
}
