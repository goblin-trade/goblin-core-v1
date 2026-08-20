use core::marker::PhantomData;

use crate::input_processor::{ArgsReader, FixedDecode};

impl<'a, T> FixedDecode<'a> for PhantomData<T> {
    const ENCODED_SIZE: usize = 0;

    fn raw_fixed_decode(_reader: &ArgsReader) -> Self {
        PhantomData
    }
}

impl<'a> FixedDecode<'a> for u8 {
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
        let offset = reader.offset.get();
        let value = reader.args[offset];
        reader.advance_offset(1);
        value
    }
}

impl<'a> FixedDecode<'a> for u16 {
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
        let offset = reader.offset.get();
        let value = u16::from_le_bytes([reader.args[offset], reader.args[offset + 1]]);
        reader.advance_offset(2);
        value
    }
}

impl<'a> FixedDecode<'a> for u32 {
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
        let offset = reader.offset.get();
        let value = u32::from_le_bytes([
            reader.args[offset],
            reader.args[offset + 1],
            reader.args[offset + 2],
            reader.args[offset + 3],
        ]);
        reader.advance_offset(4);
        value
    }
}

impl<'a> FixedDecode<'a> for u64 {
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
        let offset = reader.offset.get();
        let value = u64::from_le_bytes([
            reader.args[offset],
            reader.args[offset + 1],
            reader.args[offset + 2],
            reader.args[offset + 3],
            reader.args[offset + 4],
            reader.args[offset + 5],
            reader.args[offset + 6],
            reader.args[offset + 7],
        ]);
        reader.advance_offset(8);
        value
    }
}

impl<'a> FixedDecode<'a> for i64 {
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
        let offset = reader.offset.get();
        let value = i64::from_le_bytes([
            reader.args[offset],
            reader.args[offset + 1],
            reader.args[offset + 2],
            reader.args[offset + 3],
            reader.args[offset + 4],
            reader.args[offset + 5],
            reader.args[offset + 6],
            reader.args[offset + 7],
        ]);
        reader.advance_offset(8);
        value
    }
}
