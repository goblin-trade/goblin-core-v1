use goblin_macros::fixed_codec;

use crate::{
    axis::occupancy::OccupancyEnum,
    quantities::{BaseLots, InnerPos},
};

/// Make instruction header.
///
/// The wire layout is 5 bytes: `inner_pos` occupies the whole first byte, and
/// the remaining 32 bits hold `occupancy_enum` (1 bit), `inner_enum_raw`
/// (1 bit) and `base_lots_u32` (the remaining 30 bits). The trailing field's
/// width is inferred as `40 - 8 - 1 - 1`; the fields before the flags take
/// their full width.
#[fixed_codec(bits = 40)]
pub struct MakeHeader {
    pub inner_pos: InnerPos,
    pub occupancy_enum: OccupancyEnum,
    pub inner_enum_raw: bool,
    pub base_lots_u32: BaseLots<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_processor::{ArgsReader, ArgsWriter, FixedCodec};

    #[test]
    fn round_trip_matches_manual_layout() {
        let header = MakeHeader {
            inner_pos: InnerPos::new(0x12),
            occupancy_enum: OccupancyEnum::Occupied,
            inner_enum_raw: true,
            base_lots_u32: BaseLots::new(0x0123_4567),
        };

        // Byte 0 = inner_pos.
        // Bytes 1..5 = u32 LE: bit0 = Occupied, bit1 = inner_enum_raw,
        // bits 2..31 = base_lots.
        let region: u32 = 0b01 | (0b1 << 1) | (0x0123_4567 << 2);
        let mut expected = [0u8; 5];
        expected[0] = 0x12;
        expected[1..].copy_from_slice(&region.to_le_bytes());

        assert_eq!(MakeHeader::ENCODED_SIZE, 5);

        let mut buf = [0u8; 5];
        let mut writer = ArgsWriter::new(&mut buf);
        header.raw_fixed_encode(&mut writer);
        assert_eq!(buf, expected);

        let reader = ArgsReader::from_slice(&buf);
        let decoded = MakeHeader::try_fixed_decode(&reader).unwrap();
        assert_eq!(decoded.inner_pos.inner, 0x12);
        assert_eq!(decoded.occupancy_enum, OccupancyEnum::Occupied);
        assert!(decoded.inner_enum_raw);
        assert_eq!(decoded.base_lots_u32.inner, 0x0123_4567);
    }
}
