use crate::codec::{CodecResult, GoblinRead, Reader};

use crate::axis::leg::SamePair;
use crate::axis_helpers::TokenPair;
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};
use crate::settlement::local_delta::LocalDeposits;

use super::MarketHeader;

impl<'de, TP: TokenPair> GoblinRead<'de, ()> for MarketHeader<TP> {
    #[inline]
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let lane = read_lane::<1>(reader)?;

        let decode_deposit_amounts = unpack_bool(lane, 0);
        let local_deposits = if decode_deposit_amounts {
            LocalDeposits::<TP>::from_reader_with_ctx(reader, ())?
        } else {
            LocalDeposits::<TP>::default()
        };

        Ok(Self {
            decode_deposit_amounts,
            execute_takes: SamePair::new(unpack_bool(lane, 1), unpack_bool(lane, 2)),
            outer_bitmap_count: unpack(lane, 3, 5) as u8,
            local_deposits,
        })
    }
}
