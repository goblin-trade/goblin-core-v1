use deku::{
    DekuError, DekuReader,
    no_std_io::{Cursor, Read, Seek},
    reader::Reader,
};

/// Number of hardcoded and dynamic markets to process, as decoded from calldata.
pub struct MarketCounts {
    counts: [u8; 11],
}

impl MarketCounts {
    pub fn new(counts: [u8; 11]) -> Self {
        Self { counts }
    }

    /// A read cursor over the decoded counts, yielding one count per byte read.
    pub fn cursor(&self) -> Cursor<&[u8]> {
        Cursor::new(&self.counts[..])
    }
}

/// Each count is a nibble, packed LSB-first. Hardcoded counts always occupy
/// the first two bytes; dynamic counts add four more bytes when enabled.
impl<'a> DekuReader<'a, bool> for MarketCounts {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
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

#[cfg(feature = "encode")]
mod encode {
    use deku::{
        DekuError, DekuWriter,
        no_std_io::{Seek, Write},
        writer::Writer,
    };

    use super::MarketCounts;

    /// Write the counts back as packed nibbles: two bytes for the hardcoded
    /// counts, four more for the dynamic counts when enabled.
    impl DekuWriter<bool> for MarketCounts {
        fn to_writer<W: Write + Seek>(
            &self,
            writer: &mut Writer<W>,
            process_dynamic_markets: bool,
        ) -> Result<(), DekuError> {
            let counts = &self.counts;

            let mut bytes = [0u8; 6];
            bytes[0] = counts[0] | (counts[1] << 4);
            bytes[1] = counts[2];

            let len = if process_dynamic_markets {
                bytes[2] = counts[3] | (counts[4] << 4);
                bytes[3] = counts[5] | (counts[6] << 4);
                bytes[4] = counts[7] | (counts[8] << 4);
                bytes[5] = counts[9] | (counts[10] << 4);
                6
            } else {
                2
            };

            writer.write_bytes(&bytes[..len])
        }
    }
}
