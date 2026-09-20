use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::{
    axis::occupancy::OccupancyEnum,
    quantities::{BaseLots, InnerPos},
};

/// Make instruction header.
///
/// The wire layout is 5 bytes. base_lots_u32` is shifted 2 bits to fit
/// in `occupancy_enum` and `inner_enum_raw`
///
#[derive(DekuRead)]
#[deku(bit_order = "lsb")]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct MakeHeader {
    #[deku(
        bits = "8",
        map = "|raw: u8| -> Result<_, deku::DekuError> { Ok(InnerPos::from_raw(raw as u64)) }"
    )]
    pub inner_pos: InnerPos,
    #[deku(
        bits = "1",
        map = "|raw: u8| -> Result<_, deku::DekuError> { Ok(OccupancyEnum::from_raw(raw as u64)) }"
    )]
    pub occupancy_enum: OccupancyEnum,
    #[deku(bits = "1")]
    pub inner_enum_raw: bool,
    #[deku(
        bits = "30",
        map = "|raw: u32| -> Result<_, deku::DekuError> { Ok(BaseLots::<u32>::from_raw(raw as u64)) }"
    )]
    pub base_lots_u32: BaseLots<u32>,
}

#[cfg(test)]
mod tests {
    use deku::DekuReader;
    use deku::no_std_io::Cursor;
    use deku::reader::Reader;

    use super::*;

    #[test]
    fn decodes_lsb_bit_layout() {
        // 5 bytes, LSB-first:
        //   bits 0..7   inner_pos = 42
        //   bit  8      occupancy_enum = 1
        //   bit  9      inner_enum_raw = true
        //   bits 10..39 base_lots_u32 = (0x04 << 6) = 256
        let mut reader = Reader::new(Cursor::new(
            [42u8, 0b0000_0011, 0x04, 0x00, 0x00].as_slice(),
        ));
        let header = MakeHeader::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert_eq!(header.inner_pos.inner, 42);
        assert_eq!(header.occupancy_enum as u8, 1);
        assert!(header.inner_enum_raw);
        assert_eq!(header.base_lots_u32.inner, 256);
    }
}
