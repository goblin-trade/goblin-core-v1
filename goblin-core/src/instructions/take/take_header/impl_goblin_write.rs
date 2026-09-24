use crate::codec::{CodecResult, GoblinWrite, Writer};

use crate::axis::leg::LegMatcher;
use crate::input_processor::pack;
use crate::input_processor::pack_bool;
use crate::input_processor::write_lane;
use crate::instructions::TakeHeader;
use crate::quantities::QuantityOps;

impl<In: LegMatcher> GoblinWrite<()> for TakeHeader<In> {
    #[inline]
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        let mut lane = 0u64;
        lane = pack_bool(lane, self.flags.read_min_lots, 0);
        lane = pack_bool(lane, self.flags.read_limit, 1);
        lane = pack(lane, self.num_lots_u32.to_raw(), 2, 30);
        write_lane::<4>(writer, lane)?;

        if self.flags.read_min_lots {
            self.min_lots_to_fill_u32.to_writer(writer, ())?;
        }
        if self.flags.read_limit {
            self.limit_u32.to_writer(writer, ())?;
        }

        Ok(())
    }
}
