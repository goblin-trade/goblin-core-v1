use crate::codec::{CodecResult, GoblinWrite, Writer};

use crate::input_processor::HeaderFlags;
use crate::input_processor::bit_lane::{pack, pack_bool, write_lane};

impl GoblinWrite<()> for HeaderFlags {
    #[inline]
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        let mut lane = 0u64;
        lane = pack_bool(lane, self.read_custom_recipient, 0);
        lane = pack_bool(lane, self.read_msg_value, 1);
        lane = pack_bool(lane, self.process_dynamic_markets, 2);
        lane = pack_bool(lane, self.withdraw_eth, 3);
        lane = pack_bool(lane, self.withdraw_internally, 4);
        lane = pack(lane, self.custom_erc20_count as u64, 5, 3);

        write_lane::<1>(writer, lane)
    }
}
