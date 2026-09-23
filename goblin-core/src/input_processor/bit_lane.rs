//! Byte-aligned bit-lane helpers for the hand-rolled bit-packed decoders.
//!
//! deku's `bits` feature is disabled, so sub-byte fields are decoded by reading a
//! small little-endian lane of whole bytes and slicing it LSB-first. This mirrors
//! the wire layout the old `#[fixed_codec(bits = N)]` macro produced (a
//! little-endian lane read, then LSB-first `unpack_from(lane, shift, width)` per
//! field), and the `bit_order = "lsb"` derives it replaced.

use deku::{
    DekuError,
    ctx::Order,
    no_std_io::{Read, Seek},
    reader::Reader,
};

#[cfg(feature = "encode")]
use deku::{no_std_io::Write, writer::Writer};

/// Read an `N`-byte little-endian lane, advancing the reader by `N` bytes.
///
/// The reader must be byte aligned; every bit-packed header in this crate either
/// starts a payload or follows a whole number of bytes.
#[inline]
pub fn read_lane<R: Read + Seek, const N: usize>(reader: &mut Reader<R>) -> Result<u64, DekuError> {
    let mut buf = [0u8; N];
    reader.read_bytes_const::<N>(&mut buf, Order::Msb0)?;

    let mut lane = 0u64;
    let mut i = 0;
    while i < N {
        lane |= (buf[i] as u64) << (i * 8);
        i += 1;
    }
    Ok(lane)
}

/// Extract `bits` bits starting at `shift` from a lane, LSB-first.
#[inline]
pub const fn unpack(lane: u64, shift: u32, bits: u32) -> u64 {
    let mask = if bits >= 64 {
        u64::MAX
    } else {
        (1u64 << bits) - 1
    };
    (lane >> shift) & mask
}

/// Extract a single bit at `shift` as a `bool`.
#[inline]
pub const fn unpack_bool(lane: u64, shift: u32) -> bool {
    (lane >> shift) & 1 != 0
}

/// Pack the low `bits` bits of `value` into a lane at `shift`, LSB-first.
#[inline]
pub const fn pack(lane: u64, value: u64, shift: u32, bits: u32) -> u64 {
    let mask = if bits >= 64 {
        u64::MAX
    } else {
        (1u64 << bits) - 1
    };
    lane | ((value & mask) << shift)
}

/// Pack a single bit into a lane at `shift`.
#[inline]
pub const fn pack_bool(lane: u64, value: bool, shift: u32) -> u64 {
    lane | ((value as u64) << shift)
}

/// Write the low `N` bytes of `lane`, little-endian.
#[cfg(feature = "encode")]
#[inline]
pub fn write_lane<W: Write + Seek, const N: usize>(
    writer: &mut Writer<W>,
    lane: u64,
) -> Result<(), DekuError> {
    let mut buf = [0u8; N];
    let mut i = 0;
    while i < N {
        buf[i] = (lane >> (i * 8)) as u8;
        i += 1;
    }
    writer.write_bytes(&buf)
}
