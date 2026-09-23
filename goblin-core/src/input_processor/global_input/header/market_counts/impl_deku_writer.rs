use deku::{
    DekuError, DekuWriter,
    no_std_io::{Seek, Write},
    writer::Writer,
};

use crate::{axis::leg::Pair, axis_helpers::MarketSpec, for_axes, types::StoreReader};

use super::MarketCountsV2;

/// Write the counts back as packed nibbles, mirroring the reader: the three
/// hardcoded counts always occupy the first two bytes, and the eight dynamic
/// counts add four more bytes when `process_dynamic_markets` is set. Illegal
/// combinations are skipped.
impl DekuWriter<bool> for MarketCountsV2 {
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        process_dynamic_markets: bool,
    ) -> Result<(), DekuError> {
        let mut nibbles = [0u8; 11];
        let mut next = nibbles.iter_mut();

        for_axes!(M, TM0, TM1 => {
            if !<(M, Pair<TM0, TM1>) as MarketSpec>::illegal() {
                if let Some(nibble) = next.next() {
                    *nibble = *TM1::get_leg(TM0::get_leg(M::get_leg(self)));
                }
            }
        });

        let mut bytes = [0u8; 6];
        bytes[0] = nibbles[0] | (nibbles[1] << 4);
        bytes[1] = nibbles[2];

        let len = if process_dynamic_markets {
            for i in 0..4 {
                bytes[2 + i] = nibbles[3 + 2 * i] | (nibbles[3 + 2 * i + 1] << 4);
            }
            6
        } else {
            2
        };

        writer.write_bytes(&bytes[..len])
    }
}
