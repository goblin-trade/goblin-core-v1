use core::cell::Cell;

use deku::{DekuError, DekuReader};

use crate::input_processor::ArgsReaderV2;

pub struct MarketCounts {
    counts: [u8; 11],
    index: Cell<usize>,
}

impl MarketCounts {
    pub fn new(counts: [u8; 11]) -> Self {
        Self {
            counts,
            index: Cell::default(),
        }
    }

    fn count(&self) -> u8 {
        self.counts[self.index.get()]
    }

    pub fn get_count_and_advance(&self) -> u8 {
        let count = self.count();
        self.index.set(self.index.get() + 1);

        count
    }

    /// Each count is a nibble, packed LSB-first. Hardcoded counts always occupy
    /// the first two bytes; dynamic counts add four more bytes when enabled.
    pub fn decode(
        reader: &mut ArgsReaderV2<'_>,
        process_dynamic_markets: bool,
    ) -> Result<Self, DekuError> {
        let byte_0 = u8::from_reader_with_ctx(reader, ())?;
        let byte_1 = u8::from_reader_with_ctx(reader, ())?;

        // byte_1 has 4 free unused bits
        let hardcoded_count = [byte_0 & 0b0000_1111, byte_0 >> 4, byte_1 & 0b0000_1111];

        let custom_count = if process_dynamic_markets {
            let byte_2 = u8::from_reader_with_ctx(reader, ())?;
            let byte_3 = u8::from_reader_with_ctx(reader, ())?;
            let byte_4 = u8::from_reader_with_ctx(reader, ())?;
            let byte_5 = u8::from_reader_with_ctx(reader, ())?;

            [
                byte_2 & 0b0000_1111,
                byte_2 >> 4,
                byte_3 & 0b0000_1111,
                byte_3 >> 4,
                byte_4 & 0b0000_1111,
                byte_4 >> 4,
                byte_5 & 0b0000_1111,
                byte_5 >> 4,
            ]
        } else {
            [0u8; 8]
        };

        let mut counts = [0u8; 11];
        counts[..3].copy_from_slice(&hardcoded_count);
        counts[3..].copy_from_slice(&custom_count);

        Ok(Self::new(counts))
    }
}
