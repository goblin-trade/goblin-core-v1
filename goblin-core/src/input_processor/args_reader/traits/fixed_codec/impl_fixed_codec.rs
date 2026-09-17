use core::marker::PhantomData;

use crate::input_processor::{ArgsReader, ArgsWriter, FixedCodec};

/// Macro-implemented for the fixed-width integer types used on the wire.
///
/// `usize`/`isize` are deliberately excluded: their width is target dependent
/// (32-bit on wasm, 64-bit on the host), which makes for an unstable wire
/// format. Use a fixed-width integer and cast at the boundary instead.
macro_rules! impl_fixed_codec_int {
    ($($t:ty => $n:literal),* $(,)?) => {$(
        impl FixedCodec for $t {
            const ENCODED_SIZE: usize = $n;

            #[inline]
            fn raw_fixed_decode(reader: &ArgsReader) -> Self {
                let offset = reader.offset.get();
                let mut bytes = [0u8; $n];
                bytes.copy_from_slice(&reader.args[offset..offset + $n]);
                reader.advance_offset($n);
                Self::from_le_bytes(bytes)
            }

            #[inline]
            fn raw_fixed_encode(&self, writer: &mut ArgsWriter) {
                writer.write_bytes(&self.to_le_bytes());
            }
        }
    )*};
}

impl_fixed_codec_int! {
    u8 => 1, u16 => 2, u32 => 4, u64 => 8,
    i8 => 1, i16 => 2, i32 => 4, i64 => 8,
}

impl FixedCodec for bool {
    const ENCODED_SIZE: usize = 1;

    #[inline]
    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
        u8::raw_fixed_decode(reader) != 0
    }

    #[inline]
    fn raw_fixed_encode(&self, writer: &mut ArgsWriter) {
        (*self as u8).raw_fixed_encode(writer);
    }
}

impl<T> FixedCodec for PhantomData<T> {
    const ENCODED_SIZE: usize = 0;

    #[inline]
    fn raw_fixed_decode(_reader: &ArgsReader) -> Self {
        PhantomData
    }

    #[inline]
    fn raw_fixed_encode(&self, _writer: &mut ArgsWriter) {}
}
