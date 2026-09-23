use deku::{
    DekuError, DekuReader,
    no_std_io::{Read, Seek},
    reader::Reader,
};

use crate::{axis::leg::Pair, axis_helpers::MarketSpec, for_axes, types::StoreReader};

use super::MarketCountsV2;

/// Each count is a nibble, packed LSB-first, stored at the position of its
/// `(market, base token, quote token)` combination.
///
/// The three hardcoded counts always occupy the first two bytes; the eight
/// dynamic counts add four more bytes when `process_dynamic_markets` is set.
/// Illegal combinations have no count on the wire and stay zeroed.
impl<'a> DekuReader<'a, bool> for MarketCountsV2 {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        process_dynamic_markets: bool,
    ) -> Result<Self, DekuError> {
        let byte_0 = u8::from_reader_with_ctx(reader, ())?;
        let byte_1 = u8::from_reader_with_ctx(reader, ())?;

        // byte_1 has 4 free unused bits
        let mut nibbles = [0u8; 11];
        nibbles[0] = byte_0 & 0b0000_1111;
        nibbles[1] = byte_0 >> 4;
        nibbles[2] = byte_1 & 0b0000_1111;

        if process_dynamic_markets {
            for i in 0..4 {
                let byte = u8::from_reader_with_ctx(reader, ())?;
                nibbles[3 + 2 * i] = byte & 0b0000_1111;
                nibbles[3 + 2 * i + 1] = byte >> 4;
            }
        }

        let mut counts = MarketCountsV2::default();
        let mut nibbles = nibbles.into_iter();

        // The wire order mirrors `for_axes!`: iterate the same legal
        // `(market, base, quote)` combinations and hand each one the next nibble.
        for_axes!(M, TM0, TM1 => {
            if !<(M, Pair<TM0, TM1>) as MarketSpec>::illegal() {
                let count = nibbles.next().unwrap_or(0);
                *TM1::get_leg_mut(TM0::get_leg_mut(M::get_leg_mut(&mut counts))) = count;
            }
        });

        Ok(counts)
    }
}
