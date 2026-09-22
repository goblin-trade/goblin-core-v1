//! Zero-copy decoding on top of a deku [`Reader`].
use core::mem::size_of;

use deku::no_std_io::{Cursor, Read, Seek, SeekFrom};
use deku::reader::Reader;

/// Borrow values directly out of a deku reader's backing slice.
pub trait ZeroCopyRead<'a> {
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
    /// See [`ZeroCopyRead::zero_copy`].
    unsafe fn zero_copy_slice<T>(&mut self, slice_len: usize) -> &'a [T];
}

impl<'a> ZeroCopyRead<'a> for Reader<Cursor<&'a [u8]>> {
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
/// This is the reader-generic form of [`ZeroCopyRead::zero_copy`]. Taking the
/// backing slice explicitly lets it be called from a [`DekuReader`] impl, whose
/// reader type `R` cannot name that slice.
///
/// # Safety
///
/// See [`ZeroCopyRead::zero_copy`]; additionally, `source` must be the
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
/// See [`ZeroCopyRead::zero_copy_slice`]; additionally, `source` must be the
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
