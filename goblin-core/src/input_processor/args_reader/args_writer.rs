/// Cursor for writing fixed-size values into a caller-owned buffer.
///
/// This is the write-side counterpart to [`ArgsReader`](super::ArgsReader). It
/// is deliberately a *different* struct rather than a `&mut ArgsReader`:
///
/// * Reading uses `&ArgsReader` with an interior-mutable [`Cell`](core::cell::Cell)
///   offset so that decoding can be shared and, more importantly, so that
///   zero-copy decoders can hand out `&'a` slices into the buffer while other
///   values keep decoding. A `&mut` cursor cannot express that: once a method
///   returns `&'a T` derived from `&'a mut Reader`, the reader stays mutably
///   borrowed for `'a` and no further decode calls are possible without unsafe
///   aliasing tricks.
/// * Writing is inherently exclusive and sequential, so a plain `&mut self`
///   cursor with a `usize` offset is the natural fit. There is no zero-copy on
///   the write path to justify interior mutability.
///
/// Keeping them separate also means the encode path never has to reason about
/// `Cell` and the read path never has to reason about write bounds.
///
/// [`Cell`]: core::cell::Cell
pub struct ArgsWriter<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> ArgsWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, offset: 0 }
    }

    /// Number of bytes written so far.
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Entire backing buffer, regardless of how much has been written.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// Advance the cursor without writing. Used by codecs whose encoded size is
    /// known but whose bytes are produced out of line.
    pub fn advance_offset(&mut self, increment: usize) {
        self.offset += increment;
    }

    /// Copy `bytes` into the buffer at the current offset and advance.
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        let end = self.offset + bytes.len();
        debug_assert!(end <= self.buf.len(), "ArgsWriter overflow");
        self.buf[self.offset..end].copy_from_slice(bytes);
        self.offset = end;
    }

    /// The bytes written so far.
    pub fn as_slice(&self) -> &[u8] {
        &self.buf[..self.offset]
    }

    /// Consume the writer, returning how many bytes were written.
    pub fn finish(self) -> usize {
        self.offset
    }
}
