use crate::codec::{CodecResult, GoblinRead, Reader};

use crate::{axis::leg::Pair, axis_helpers::MarketSpec, for_axes, types::StoreReader};

use super::MarketCounts;

/// Each count is a nibble, packed LSB-first, stored at the position of its
/// `(market, base token, quote token)` combination.
///
/// The three hardcoded counts always occupy the first two bytes; the eight
/// dynamic counts add four more bytes when `process_dynamic_markets` is set.
/// Illegal combinations have no count on the wire and stay zeroed.
impl<'de> GoblinRead<'de, bool> for MarketCounts {
    fn from_reader_with_ctx(
        reader: &mut Reader<'de>,
        process_dynamic_markets: bool,
    ) -> CodecResult<Self> {
        let byte_0 = reader.read_u8()?;
        let byte_1 = reader.read_u8()?;

        // byte_1 has 4 free unused bits
        let mut nibbles = [0u8; 11];
        nibbles[0] = byte_0 & 0b0000_1111;
        nibbles[1] = byte_0 >> 4;
        nibbles[2] = byte_1 & 0b0000_1111;

        if process_dynamic_markets {
            for i in 0..4 {
                let byte = reader.read_u8()?;
                nibbles[3 + 2 * i] = byte & 0b0000_1111;
                nibbles[3 + 2 * i + 1] = byte >> 4;
            }
        }

        let mut counts = MarketCounts::default();
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
