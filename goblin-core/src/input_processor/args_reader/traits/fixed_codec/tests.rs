use core::marker::PhantomData;

use goblin_macros::fixed_codec;

use crate::goblin_error::GoblinError;
use crate::input_processor::{ArgsReader, ArgsWriter, BitPack, FixedCodec, bit_mask};

/// Sub-byte fields followed by a byte-aligned field. The five leading bools and
/// the two 1-bit/2-bit fields fill exactly one byte, so the wire layout is
/// `[lane:u8, count:u16]`.
#[fixed_codec]
#[derive(Debug, PartialEq, Clone, Copy)]
struct ByteThenWord {
    #[codec(bits = 1)]
    flag_a: bool,
    #[codec(bits = 1)]
    flag_b: bool,
    #[codec(bits = 2)]
    small: u8,
    #[codec(bits = 4)]
    other: u8,
    count: u16,
}

#[fixed_codec]
#[derive(Debug, PartialEq, Clone, Copy)]
struct TuplePair(#[codec(bits = 3)] u8, #[codec(bits = 5)] u8);

/// Top-level size: each `bool` is one bit and the trailing field absorbs the
/// remaining `8 - 3 = 5` bits.
#[fixed_codec(bits = 8)]
#[derive(Debug, PartialEq, Clone, Copy)]
struct SizedStruct {
    a: bool,
    b: bool,
    c: bool,
    value: u8,
}

/// A sub-byte run wider than one byte promotes to the next lane type.
#[fixed_codec]
#[derive(Debug, PartialEq, Clone, Copy)]
struct WideLane {
    #[codec(bits = 12)]
    a: u16,
    #[codec(bits = 20)]
    b: u32,
}

#[fixed_codec]
#[derive(Debug, PartialEq, Clone, Copy)]
struct Generic<T> {
    value: u8,
    _marker: PhantomData<T>,
}

/// A 2-bit flag pair used as an inner `BitPack` field by [`Packed`].
#[derive(Debug, PartialEq, Clone, Copy)]
struct Flags {
    a: bool,
    b: bool,
}

impl BitPack for Flags {
    const CAPACITY: u8 = 2;

    fn to_raw(self) -> u64 {
        (self.a as u64) | ((self.b as u64) << 1)
    }

    fn from_raw(raw: u64) -> Self {
        Self {
            a: raw & 1 != 0,
            b: raw & 0b10 != 0,
        }
    }
}

/// `validate = Self::check` keeps a struct-level invariant while still deriving
/// the wire codec.
#[fixed_codec(bits = 32, validate = Self::check)]
#[derive(Debug, PartialEq, Clone, Copy)]
struct Packed {
    flags: Flags,
    lots: u32,
}

impl Packed {
    fn check(&self) -> Result<(), GoblinError> {
        if self.lots > 0 {
            Ok(())
        } else {
            Err(GoblinError::InvalidTakeArgs)
        }
    }
}

/// `#[codec(wire = u8)]` lets a `usize` field encode as a single byte.
#[fixed_codec(validate = Self::check)]
#[derive(Debug, PartialEq, Clone, Copy)]
struct SmallIndex(#[codec(wire = u8)] pub usize);

impl SmallIndex {
    fn check(&self) -> Result<(), GoblinError> {
        if self.0 <= 3 {
            Ok(())
        } else {
            Err(GoblinError::InvalidPayload)
        }
    }
}

fn round_trip<T>(value: T) -> T
where
    T: FixedCodec + PartialEq + core::fmt::Debug,
{
    let mut buf = [0u8; 64];
    let written = {
        let mut writer = ArgsWriter::new(&mut buf);
        value.raw_fixed_encode(&mut writer);
        writer.offset()
    };
    assert_eq!(written, T::ENCODED_SIZE, "encoded size mismatch");

    let reader = ArgsReader::from_slice(&buf[..written]);
    let decoded = T::try_fixed_decode(&reader).expect("decode");
    assert_eq!(reader.offset.get(), T::ENCODED_SIZE, "cursor not advanced");
    decoded
}

#[test]
fn byte_then_word_layout() {
    let value = ByteThenWord {
        flag_a: true,
        flag_b: false,
        small: 0b10,
        other: 0b1010,
        count: 0x1234,
    };

    // LSB-first: bit0=flag_a, bit1=flag_b, bits2..3=small, bits4..7=other.
    let expected_lane = 0b0000_0001 | (0b10 << 2) | (0b1010 << 4);
    let expected = [expected_lane, 0x34, 0x12];

    let mut buf = [0u8; 3];
    let mut writer = ArgsWriter::new(&mut buf);
    value.raw_fixed_encode(&mut writer);
    assert_eq!(buf, expected);
    assert_eq!(ByteThenWord::ENCODED_SIZE, 3);

    assert_eq!(round_trip(value), value);
}

#[test]
fn top_level_size_infers_remaining_width() {
    let value = SizedStruct {
        a: true,
        b: false,
        c: true,
        value: 0b10101,
    };

    assert_eq!(SizedStruct::ENCODED_SIZE, 1);

    // a=bit0, b=bit1, c=bit2, value=bits3..7.
    let expected = 0b1010_1101;
    let mut buf = [0u8; 1];
    let mut writer = ArgsWriter::new(&mut buf);
    value.raw_fixed_encode(&mut writer);
    assert_eq!(buf[0], expected);

    assert_eq!(round_trip(value), value);
}

#[test]
fn tuple_struct_bit_pack() {
    let value = TuplePair(0b101, 0b11001);
    let mut buf = [0u8; 1];
    let mut writer = ArgsWriter::new(&mut buf);
    value.raw_fixed_encode(&mut writer);
    assert_eq!(buf[0], 0b1100_1101);
    assert_eq!(round_trip(value), value);
}

#[test]
fn wide_lane_promotes() {
    // 12 + 20 = 32 bits => a single u32 lane.
    assert_eq!(WideLane::ENCODED_SIZE, 4);
    let value = WideLane {
        a: 0xABC,
        b: 0xFEDCB,
    };
    assert_eq!(round_trip(value), value);
}

#[test]
fn generics_and_phantom_data() {
    assert_eq!(Generic::<u64>::ENCODED_SIZE, 1);
    let value = Generic::<u64> {
        value: 42,
        _marker: PhantomData,
    };
    assert_eq!(round_trip(value), value);
}

#[test]
fn validate_hook_runs_on_decode() {
    let value = Packed {
        flags: Flags { a: true, b: false },
        lots: 0b101,
    };

    // flags at bits 0..2, lots at bits 2..32.
    assert_eq!(Packed::ENCODED_SIZE, 4);
    let mut buf = [0u8; 4];
    let mut writer = ArgsWriter::new(&mut buf);
    value.raw_fixed_encode(&mut writer);
    assert_eq!(buf, [0b0001_0101, 0, 0, 0]);
    assert_eq!(round_trip(value), value);

    // The custom `validate` rejects `lots == 0` on decode.
    let invalid = Packed {
        flags: Flags { a: false, b: false },
        lots: 0,
    };
    let mut buf = [0u8; 4];
    let mut writer = ArgsWriter::new(&mut buf);
    invalid.raw_fixed_encode(&mut writer);
    let reader = ArgsReader::from_slice(&buf);
    assert!(Packed::try_fixed_decode(&reader).is_err());
}

#[test]
fn wire_type_override_for_usize() {
    assert_eq!(SmallIndex::ENCODED_SIZE, 1);
    assert_eq!(round_trip(SmallIndex(2)), SmallIndex(2));

    let mut buf = [0u8; 1];
    let mut writer = ArgsWriter::new(&mut buf);
    SmallIndex(3).raw_fixed_encode(&mut writer);
    assert_eq!(buf[0], 3);

    // The custom `validate` rejects anything above 3.
    let mut buf = [0u8; 1];
    let mut writer = ArgsWriter::new(&mut buf);
    SmallIndex(4).raw_fixed_encode(&mut writer);
    let reader = ArgsReader::from_slice(&buf);
    assert!(SmallIndex::try_fixed_decode(&reader).is_err());
}

#[test]
fn bit_pack_helpers() {
    let mut lane = 0u64;
    0b101u8.pack_into(&mut lane, 0, 3);
    0b11u8.pack_into(&mut lane, 3, 2);
    assert_eq!(lane, 0b0001_1101);
    assert_eq!(u8::unpack_from(lane, 0, 3), 0b101);
    assert_eq!(u8::unpack_from(lane, 3, 2), 0b11);
    assert!(bool::unpack_from(0b1, 0, 1));
    assert_eq!(bit_mask(0), 0);
    assert_eq!(bit_mask(64), u64::MAX);
}
