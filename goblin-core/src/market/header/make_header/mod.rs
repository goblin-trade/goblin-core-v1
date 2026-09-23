#[cfg(feature = "encode")]
use deku::DekuWriter;
#[cfg(feature = "encode")]
use deku::no_std_io::Write;
use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;
#[cfg(feature = "encode")]
use deku::writer::Writer;
use deku::{DekuError, DekuReader};

#[cfg(feature = "encode")]
use crate::input_processor::bit_lane::{pack, pack_bool, write_lane};
use crate::{
    axis::occupancy::OccupancyEnum,
    input_processor::bit_lane::{read_lane, unpack, unpack_bool},
    quantities::{BaseLots, InnerPos},
};

/// Make instruction header.
///
/// The wire layout is 5 bytes. `base_lots_u32` is shifted 2 bits to fit
/// in `occupancy_enum` and `inner_enum_raw`. deku's `bits` feature is off, so
/// the five bytes are decoded as a single LSB-first lane.
pub struct MakeHeader {
    pub inner_pos: InnerPos,

    pub occupancy_enum: OccupancyEnum,

    pub inner_enum_raw: bool,

    pub base_lots_u32: BaseLots<u32>,
}

impl<'a> DekuReader<'a, ()> for MakeHeader {
    #[inline]
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        _ctx: (),
    ) -> Result<Self, DekuError> {
        let lane = read_lane::<R, 5>(reader)?;

        Ok(Self {
            inner_pos: InnerPos::from_raw(unpack(lane, 0, 8)),
            occupancy_enum: OccupancyEnum::from_raw(unpack(lane, 8, 1)),
            inner_enum_raw: unpack_bool(lane, 9),
            base_lots_u32: BaseLots::from_raw(unpack(lane, 10, 30)),
        })
    }
}

#[cfg(feature = "encode")]
impl DekuWriter<()> for MakeHeader {
    #[inline]
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        _ctx: (),
    ) -> Result<(), DekuError> {
        let mut lane = 0u64;
        lane = pack(lane, self.inner_pos.inner as u64, 0, 8);
        lane = pack(lane, self.occupancy_enum as u64, 8, 1);
        lane = pack_bool(lane, self.inner_enum_raw, 9);
        lane = pack(lane, self.base_lots_u32.inner as u64, 10, 30);

        write_lane::<W, 5>(writer, lane)
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
        // lane 0x48D312 little-endian packs inner_pos (8 bits = 0x12) |
        // occupancy (1) | inner_enum_raw (1) | base_lots_u32 (30 = 0x1234).
        let bytes = [0x12u8, 0xD3, 0x48, 0x00, 0x00];
        let mut reader = Reader::new(Cursor::new(&bytes[..]));
        let header = MakeHeader::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert_eq!(header.inner_pos.inner, 0x12);
        assert_eq!(header.occupancy_enum, OccupancyEnum::Occupied);
        assert!(header.inner_enum_raw);
        assert_eq!(header.base_lots_u32.inner, 0x1234);
    }
}
