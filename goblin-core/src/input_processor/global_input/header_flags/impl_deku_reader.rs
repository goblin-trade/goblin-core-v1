use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;
use deku::{DekuError, DekuReader};

use crate::input_processor::HeaderFlags;
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};

impl<'a> DekuReader<'a, ()> for HeaderFlags {
    #[inline]
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        _ctx: (),
    ) -> Result<Self, DekuError> {
        let lane = read_lane::<R, 1>(reader)?;

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
