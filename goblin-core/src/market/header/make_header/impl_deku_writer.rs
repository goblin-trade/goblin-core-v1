use deku::DekuError;
use deku::DekuWriter;
use deku::no_std_io::{Seek, Write};
use deku::writer::Writer;

use crate::input_processor::bit_lane::{pack, pack_bool, write_lane};

use super::MakeHeader;

impl DekuWriter<()> for MakeHeader {
    #[inline]
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        _ctx: (),
    ) -> Result<(), DekuError> {
        let mut lane = 0u64;
        lane = pack(lane, self.inner_pos.inner as u64, 0, 8);
        lane = pack(lane, self.occupancy_enum as u64, 8, 1);
        lane = pack_bool(lane, self.inner_enum_raw, 9);
        lane = pack(lane, self.base_lots_u32.inner as u64, 10, 30);

        write_lane::<W, 5>(writer, lane)
    }
}
