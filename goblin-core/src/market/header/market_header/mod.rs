#[cfg(feature = "encode")]
use deku::DekuWriter;
#[cfg(feature = "encode")]
use deku::no_std_io::Write;
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;
#[cfg(feature = "encode")]
use deku::writer::Writer;
use deku::{DekuError, DekuReader};

use crate::axis::leg::SamePair;
#[cfg(feature = "encode")]
use crate::input_processor::bit_lane::{pack, pack_bool, write_lane};
use crate::input_processor::bit_lane::{read_lane, unpack, unpack_bool};

/// Per-market header, packed into a single byte LSB-first.
///
/// deku's `bits` feature is off, so the byte is decoded as a lane: the trailing
/// `outer_bitmap_count` takes whatever is left after the flag fields (5 bits).
pub struct MarketHeader {
    /// Whether to decode deposit amounts
    pub decode_deposit_amounts: bool,

    /// Whether to execute take orders for sides In=Base and In=Quote
    pub execute_takes: SamePair<bool>,

    /// Number of outer bitmaps to traverse
    pub outer_bitmap_count: u8,
}

impl<'a> DekuReader<'a, ()> for MarketHeader {
    #[inline]
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        _ctx: (),
    ) -> Result<Self, DekuError> {
        let lane = read_lane::<R, 1>(reader)?;

        Ok(Self {
            decode_deposit_amounts: unpack_bool(lane, 0),
            execute_takes: SamePair::new(unpack_bool(lane, 1), unpack_bool(lane, 2)),
            outer_bitmap_count: unpack(lane, 3, 5) as u8,
        })
    }
}

#[cfg(feature = "encode")]
impl DekuWriter<()> for MarketHeader {
    #[inline]
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        _ctx: (),
    ) -> Result<(), DekuError> {
        let mut lane = 0u64;
        lane = pack_bool(lane, self.decode_deposit_amounts, 0);
        lane = pack_bool(lane, self.execute_takes.0, 1);
        lane = pack_bool(lane, self.execute_takes.1, 2);
        lane = pack(lane, self.outer_bitmap_count as u64, 3, 5);

        write_lane::<W, 1>(writer, lane)
    }
}

#[cfg(test)]
mod tests {
    use deku::DekuReader;
    use deku::no_std_io::Cursor;
    use deku::reader::Reader;

    use super::*;

    #[test]
    fn lsb_lane_layout() {
        // bit0 = decode_deposit_amounts, bit1 = base take, bit2 = quote take,
        // bits3..8 = outer_bitmap_count. 0b1010_1011 => count 0b10101 = 21.
        let byte = [0b1010_1011u8];
        let mut reader = Reader::new(Cursor::new(&byte[..]));
        let header = MarketHeader::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert!(header.decode_deposit_amounts);
        assert!(header.execute_takes.0);
        assert!(!header.execute_takes.1);
        assert_eq!(header.outer_bitmap_count, 21);
    }
}
