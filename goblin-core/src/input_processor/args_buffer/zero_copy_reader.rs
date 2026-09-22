//! Zero-copy decoding on top of a deku [`Reader`].
use core::mem::size_of;

use deku::no_std_io::{Read, Seek, SeekFrom};
use deku::reader::Reader;

/// A reader paired with the slice that backs it.
///
/// A [`DekuReader`] impl is generic over the reader `R`, so it cannot name the
/// backing `&[u8]` that zero-copy borrowing needs. Threading the slice alongside
/// the reader lets that generic path borrow through the same [`ZeroCopyRead`]
/// trait as the concrete `Reader<Cursor<&[u8]>>` impl above.
///
/// [`DekuReader`]: deku::DekuReader
pub struct ZeroCopyReader<'a, 'r, R: Read + Seek> {
    reader: &'r mut Reader<R>,
    source: &'a [u8],
}

impl<'a, 'r, R: Read + Seek> ZeroCopyReader<'a, 'r, R> {
    /// Pair `reader` with the `source` slice that backs it.
    pub fn new(reader: &'r mut Reader<R>, source: &'a [u8]) -> Self {
        Self { reader, source }
    }

    /// Reinterpret the next `size_of::<T>()` bytes as a `&'a T`, advancing the
    /// reader past them. The current offset is assumed to be byte aligned.
    ///
    /// # Safety
    ///
    /// Every bit pattern of `T` must be a valid value — e.g. integers, byte
    /// arrays, or `#[repr(C)]` aggregates built only from such types — and the
    /// bytes at the current offset must be in range.
    pub unsafe fn zero_copy<T>(&mut self) -> &'a T {
        let start = self.reader.bits_read / 8;
        let end = start + size_of::<T>();
        advance(&mut *self.reader, end - start);

        // SAFETY: validity of the bytes is the caller's contract.
        unsafe { &*(self.source[start..end].as_ptr() as *const T) }
    }

    /// Reinterpret the next `slice_len * size_of::<T>()` bytes as a `&'a [T]`,
    /// advancing the reader past them. The current offset is assumed to be byte
    /// aligned.
    ///
    /// # Safety
    ///
    /// See [`ZeroCopyRead::zero_copy`].
    pub unsafe fn zero_copy_slice<T>(&mut self, slice_len: usize) -> &'a [T] {
        let start = self.reader.bits_read / 8;
        let end = start + slice_len * size_of::<T>();
        advance(&mut *self.reader, end - start);

        // SAFETY: validity of the bytes is the caller's contract.
        unsafe {
            core::slice::from_raw_parts(self.source[start..end].as_ptr() as *const T, slice_len)
        }
    }
}

/// Advance both the logical bit count and the inner cursor by `bytes`.
fn advance<R: Read + Seek>(reader: &mut Reader<R>, bytes: usize) {
    reader
        .seek(SeekFrom::Current(bytes as i64))
        .expect("seeking the backing reader cannot fail");
}
