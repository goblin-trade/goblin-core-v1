use crate::codec::{CodecResult, GoblinWrite, Writer};

use crate::axis_helpers::MarketSpec;
use crate::input_processor::bit_lane::{pack, pack_bool, write_lane};

use super::MarketHeader;

impl<MS: MarketSpec> GoblinWrite<()> for MarketHeader<MS>
where
    MS::Locator: GoblinWrite<()>,
{
    #[inline]
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        let mut lane = 0u64;
        lane = pack_bool(lane, self.decode_deposit_amounts, 0);
        lane = pack_bool(lane, self.execute_takes.0, 1);
        lane = pack_bool(lane, self.execute_takes.1, 2);
        lane = pack(lane, self.outer_bitmap_count as u64, 3, 5);

        write_lane::<1>(writer, lane)?;

        self.locator.to_writer(writer, ())?;

        if self.decode_deposit_amounts {
            self.local_deposits.to_writer(writer, ())?;
        }

        Ok(())
    }
}
