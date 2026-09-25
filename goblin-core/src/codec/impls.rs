//! Built-in implementations of [`GoblinRead`]/[`GoblinWrite`] for the integer
//! and byte-array primitives, with the unit context.

use super::{CodecError, CodecResult, GoblinRead, GoblinWrite, Reader, Writer};

macro_rules! impl_unsigned {
    ($($t:ty => $read:ident, $write:ident;)*) => {
        $(
            impl<'de> GoblinRead<'de, ()> for $t {
                fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
                    reader.$read()
                }
            }

            impl GoblinWrite<()> for $t {
                fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
                    writer.$write(*self)
                }
            }
        )*
    };
}

impl_unsigned! {
    u8 => read_u8, write_u8;
    u16 => read_u16_le, write_u16_le;
    u32 => read_u32_le, write_u32_le;
    u64 => read_u64_le, write_u64_le;
}

macro_rules! impl_signed {
    ($($t:ty => $wide:ty, $read:ident, $write:ident;)*) => {
        $(
            impl<'de> GoblinRead<'de, ()> for $t {
                fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
                    Ok(reader.$read()? as $wide as $t)
                }
            }

            impl GoblinWrite<()> for $t {
                fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
                    writer.$write(*self as $wide)
                }
            }
        )*
    };
}

impl_signed! {
    i8 => u8, read_u8, write_u8;
    i16 => u16, read_u16_le, write_u16_le;
    i32 => u32, read_u32_le, write_u32_le;
    i64 => u64, read_u64_le, write_u64_le;
}

impl<'de, const N: usize> GoblinRead<'de, ()> for [u8; N] {
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        reader
            .take(N)?
            .try_into()
            .map_err(|_| CodecError::UnexpectedEof)
    }
}

impl<const N: usize> GoblinWrite<()> for [u8; N] {
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        writer.write(self)
    }
}
