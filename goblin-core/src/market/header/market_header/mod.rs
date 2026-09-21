use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::axis::leg::SamePair;

#[derive(DekuRead)]
#[deku(bit_order = "lsb")]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct MarketHeader {
    /// Whether to decode deposit amounts
    #[deku(bits = "1")]
    pub decode_deposit_amounts: bool,

    /// Whether to execute take orders for sides In=Base and In=Quote
    #[deku(bits = "2", ctx = "(1, 1)")]
    pub execute_takes: SamePair<bool>,

    /// Number of outer bitmaps to traverse
    #[deku(bits = "5")]
    pub outer_bitmap_count: u8,
}

#[cfg(test)]
mod tests {
    use deku::DekuReader;
    use deku::no_std_io::Cursor;
    use deku::reader::Reader;

    use super::*;

    #[test]
    fn decodes_lsb_bit_layout() {
        // 0b1010_1101, LSB-first:
        //   bit 0    decode_deposit_amounts = true
        //   bits 1..2 execute_takes = 0b10
        //   bits 3..7 outer_bitmap_count = 0b10101 = 21
        let mut reader = Reader::new(Cursor::new([0b1010_1101u8].as_slice()));
        let header = MarketHeader::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert!(header.decode_deposit_amounts);
        assert!(!header.execute_takes.0);
        assert!(header.execute_takes.1);
        assert_eq!(header.outer_bitmap_count, 21);
    }
}
