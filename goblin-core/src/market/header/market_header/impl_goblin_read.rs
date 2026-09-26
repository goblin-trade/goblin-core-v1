use crate::codec::{CodecError, CodecResult, GoblinRead, Reader};

use crate::axis::{leg::SamePair, market::MarketLocator, token::TokenDataTriple};
use crate::axis_helpers::MarketSpec;
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};
use crate::settlement::local_delta::LocalDeposits;

use super::MarketHeader;

impl<'de, 'a, 't, MS: MarketSpec> GoblinRead<'de, &'a TokenDataTriple<'t>> for MarketHeader<MS> {
    #[inline]
    fn from_reader_with_ctx(
        reader: &mut Reader<'de>,
        token_data_triple: &'a TokenDataTriple<'t>,
    ) -> CodecResult<Self> {
        let lane = read_lane::<1>(reader)?;

        let decode_deposit_amounts = unpack_bool(lane, 0);
        let locator = MS::Market::decode_locator(reader, token_data_triple)
            .map_err(|_| CodecError::InvalidValue)?;
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
