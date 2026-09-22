//! Zero-copy decoding on top of a deku [`Reader`].
//!
//! deku's derive macros, `DekuReader` implementations and `#[deku(reader = ..)]`
//! hooks are all generic over the backing source `R: Read + Seek`, so none of
//! them can name the buffer lifetime or recover its original slice. Zero-copy
//! therefore cannot live inside a derive-generated path; it is exposed here as
//! an extension trait on the concrete reader used by the program:
//! [`ArgsReaderV2`](crate::input_processor::ArgsReaderV2) =
//! `Reader<Cursor<&'a [u8]>>`.
//!
//! When the backing slice is threaded in through a Deku `ctx` instead, the
//! reader-generic [`zero_copy_from`] / [`zero_copy_slice_from`] helpers can be
//! called from a `DekuReader` impl without naming `R`.
//!
//! Like the old fixed-codec decoder, this is an unchecked primitive that
//! validates nothing: an out-of-range read panics on the slice index, and the
//! caller owns the offset, bounds and layout invariants.
//!
//! Offsets are taken from [`Reader::bits_read`], not from the cursor position.
//! When a bit-level decode leaves `leftover` bits, the inner cursor runs up to
//! one byte ahead of the logical position, so only `bits_read` is authoritative.

use core::mem::size_of;

use deku::no_std_io::{Cursor, Read, Seek, SeekFrom};
use deku::reader::Reader;

/// Borrow values directly out of a deku reader's backing slice.
pub trait ZeroCopyReadV2<'a> {
    /// Reinterpret the next `size_of::<T>()` bytes as a `&'a T`, advancing the
    /// reader past them. The current offset is assumed to be byte aligned.
    ///
    /// # Safety
    ///
    /// Every bit pattern of `T` must be a valid value — e.g. integers, byte
    /// arrays, or `#[repr(C)]` aggregates built only from such types — and the
    /// bytes at the current offset must be in range.
    unsafe fn zero_copy<T>(&mut self) -> &'a T;

    /// Reinterpret the next `slice_len * size_of::<T>()` bytes as a `&'a [T]`,
    /// advancing the reader past them. The current offset is assumed to be byte
    /// aligned.
    ///
    /// # Safety
    ///
    /// See [`ZeroCopyReadV2::zero_copy`].
    unsafe fn zero_copy_slice<T>(&mut self, slice_len: usize) -> &'a [T];
}

impl<'a> ZeroCopyReadV2<'a> for Reader<Cursor<&'a [u8]>> {
    unsafe fn zero_copy<T>(&mut self) -> &'a T {
        let source: &'a [u8] = self.as_mut().get_ref();
        // SAFETY: `source` is the reader's own backing slice.
        unsafe { zero_copy_from(self, source) }
    }

    unsafe fn zero_copy_slice<T>(&mut self, slice_len: usize) -> &'a [T] {
        let source: &'a [u8] = self.as_mut().get_ref();
        // SAFETY: `source` is the reader's own backing slice.
        unsafe { zero_copy_slice_from(self, source, slice_len) }
    }
}

/// Reinterpret the next `size_of::<T>()` bytes of `source` — starting at the
/// reader's current byte offset — as `&'a T`, advancing the reader past them.
///
/// This is the reader-generic form of [`ZeroCopyReadV2::zero_copy`]. Taking the
/// backing slice explicitly lets it be called from a [`DekuReader`] impl, whose
/// reader type `R` cannot name that slice.
///
/// # Safety
///
/// See [`ZeroCopyReadV2::zero_copy`]; additionally, `source` must be the
/// reader's own backing slice.
///
/// [`DekuReader`]: deku::DekuReader
pub unsafe fn zero_copy_from<'a, T, R: Read + Seek>(
    reader: &mut Reader<R>,
    source: &'a [u8],
) -> &'a T {
    let start = reader.bits_read / 8;
    let end = start + size_of::<T>();
    advance(reader, end - start);

    // SAFETY: validity of the bytes is the caller's contract.
    unsafe { &*(source[start..end].as_ptr() as *const T) }
}

/// Slice counterpart of [`zero_copy_from`].
///
/// # Safety
///
/// See [`ZeroCopyReadV2::zero_copy_slice`]; additionally, `source` must be the
/// reader's own backing slice.
pub unsafe fn zero_copy_slice_from<'a, T, R: Read + Seek>(
    reader: &mut Reader<R>,
    source: &'a [u8],
    slice_len: usize,
) -> &'a [T] {
    let start = reader.bits_read / 8;
    let end = start + slice_len * size_of::<T>();
    advance(reader, end - start);

    // SAFETY: validity of the bytes is the caller's contract.
    unsafe { core::slice::from_raw_parts(source[start..end].as_ptr() as *const T, slice_len) }
}

/// Advance both the logical bit count and the inner cursor by `bytes`.
fn advance<R: Read + Seek>(reader: &mut Reader<R>, bytes: usize) {
    reader
        .seek(SeekFrom::Current(bytes as i64))
        .expect("seeking the backing reader cannot fail");
}

#[cfg(test)]
mod tests {
    use deku::no_std_io::Cursor;
    use deku::reader::Reader;

    use super::*;
    use crate::input_processor::ArgsReaderV2;

    #[test]
    fn reinterprets_typed_values_in_place() {
        let data: [u8; 64] = core::array::from_fn(|i| i as u8);
        let mut reader: ArgsReaderV2 = Reader::new(Cursor::new(&data[..]));

        // SAFETY: `u32`/`u8` accept every bit pattern and stay in range.
        let value = unsafe { reader.zero_copy::<u32>() };
        assert_eq!(*value, u32::from_le_bytes([0, 1, 2, 3]));

        let tail = unsafe { reader.zero_copy_slice::<u8>(3) };
        assert_eq!(tail, &[4, 5, 6]);

        assert_eq!(reader.bits_read, 7 * 8);
    }

    #[test]
    fn generic_helpers_match_the_trait() {
        let data: [u8; 64] = core::array::from_fn(|i| i as u8);

        let mut via_trait: ArgsReaderV2 = Reader::new(Cursor::new(&data[..]));
        // SAFETY: `u32` accepts every bit pattern and stays in range.
        let expected = unsafe { via_trait.zero_copy::<u32>() };

        let mut via_helper: Reader<Cursor<&[u8]>> = Reader::new(Cursor::new(&data[..]));
        let source: &[u8] = via_helper.as_mut().get_ref();
        // SAFETY: `source` is the reader's backing slice.
        let actual = unsafe { zero_copy_from::<u32, _>(&mut via_helper, source) };

        assert_eq!(actual, expected);
        assert_eq!(via_helper.bits_read, via_trait.bits_read);
    }

    #[test]
    #[should_panic]
    fn read_past_end_panics() {
        let data: [u8; 4] = [0; 4];
        let mut reader: ArgsReaderV2 = Reader::new(Cursor::new(&data[..]));

        let _ = unsafe { reader.zero_copy_slice::<u32>(2) };
    }
}
