//! Zero-copy decode: borrow values straight out of a reader's buffer.
//!
//! Two flavors live here:
//!
//! * [`ZeroCopyReader`] works with the program's own [`ArgsReader`], which owns a
//!   fixed `[u8; INPUT_SIZE]` buffer and keeps its offset in a `Cell`.
//! * [`ZeroCopyRead`] works with deku's [`Reader`], used by the `ArgsReaderV2`
//!   migration path. See its module for why it is a separate trait.
//!
//! [`ArgsReader`]: crate::input_processor::ArgsReader
//! [`Reader`]: deku::reader::Reader

mod impl_zero_copy_reader;
mod zero_copy_read_v2;

pub use zero_copy_read_v2::*;

/// Borrow values directly from a reader's buffer without copying.
pub trait ZeroCopyRead {
    /// Reinterpret the next `core::mem::size_of::<T>()` bytes as a `&T`,
    /// advancing the reader past them.
    ///
    /// The caller must ensure the bytes are a valid `T`.
    fn zero_copy_unchecked<T>(&self) -> &T;

    /// Reinterpret the next `slice_len * core::mem::size_of::<T>()` bytes as a
    /// `&[T]`, advancing the reader past them.
    ///
    /// The caller must ensure the bytes are valid `T`s.
    fn zero_copy_slice_unchecked<T>(&self, slice_len: usize) -> &[T];
}
