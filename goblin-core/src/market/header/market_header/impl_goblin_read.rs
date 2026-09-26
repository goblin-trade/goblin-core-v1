use crate::codec::{CodecResult, GoblinRead, Reader};

use crate::axis::leg::SamePair;
use crate::axis_helpers::MarketSpec;
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};
use crate::settlement::local_delta::LocalDeposits;

use super::MarketHeader;

impl<'de, MS: MarketSpec> GoblinRead<'de, ()> for MarketHeader<MS> {
    #[inline]
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let lane = read_lane::<1>(reader)?;

        let decode_deposit_amounts = unpack_bool(lane, 0);
        let locator = MS::Locator::from_reader_with_ctx(reader, ())?;
        let local_deposits = if decode_deposit_amounts {
            LocalDeposits::<MS::Pair>::from_reader_with_ctx(reader, ())?
        } else {
            LocalDeposits::<MS::Pair>::default()
        };

        Ok(Self {
            decode_deposit_amounts,
            execute_takes: SamePair::new(unpack_bool(lane, 1), unpack_bool(lane, 2)),
            outer_bitmap_count: unpack(lane, 3, 5) as u8,
            locator,
            local_deposits,
        })
    }
}
