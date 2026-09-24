use crate::codec::{CodecResult, GoblinRead, Reader};

use crate::axis::leg::SamePair;
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};

use super::MarketHeader;

impl<'de> GoblinRead<'de, ()> for MarketHeader {
    #[inline]
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let lane = read_lane::<1>(reader)?;

        Ok(Self {
            decode_deposit_amounts: unpack_bool(lane, 0),
            execute_takes: SamePair::new(unpack_bool(lane, 1), unpack_bool(lane, 2)),
            outer_bitmap_count: unpack(lane, 3, 5) as u8,
        })
    }
}
