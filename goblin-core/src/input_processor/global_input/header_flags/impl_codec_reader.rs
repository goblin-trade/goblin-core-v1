use crate::codec::{CodecResult, GoblinRead, Reader};

use crate::input_processor::HeaderFlags;
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};

impl<'de> GoblinRead<'de, ()> for HeaderFlags {
    #[inline]
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let lane = read_lane::<1>(reader)?;

        Ok(Self {
            read_custom_recipient: unpack_bool(lane, 0),
            read_msg_value: unpack_bool(lane, 1),
            process_dynamic_markets: unpack_bool(lane, 2),
            withdraw_eth: unpack_bool(lane, 3),
            withdraw_internally: unpack_bool(lane, 4),
            custom_erc20_count: unpack(lane, 5, 3) as usize,
        })
    }
}
