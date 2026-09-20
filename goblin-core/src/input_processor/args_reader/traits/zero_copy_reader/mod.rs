//! Zero-copy decode: borrow values straight out of a reader's buffer.
//!
//! Unlike [`FixedCodec`](super::FixedCodec) and
//! [`VariableDecode`](super::VariableDecode), which produce owned values, this
//! trait hands back references into the reader's backing bytes. It therefore
//! performs no copy and no validation: the caller is responsible for knowing
//! that the bytes at the current offset form a valid `T`.
//!
//! The trait carries no lifetime of its own: borrows are tied to the `&self`
//! receiver, so the returned reference cannot outlive the reader.

mod impl_zero_copy_reader;

/// Borrow values directly from a reader's buffer without copying.
pub trait ZeroCopyReader {
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
