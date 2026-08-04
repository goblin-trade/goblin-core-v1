use core::marker::PhantomData;

use crate::input_processor::{DecodeCtx, FixedDecode};

impl<'a, T> FixedDecode<'a> for PhantomData<T> {
    const ENCODED_SIZE: usize = 0;

    fn decode_raw(_ctx: &DecodeCtx) -> Self {
        PhantomData
    }
}

impl<'a> FixedDecode<'a> for u8 {
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn decode_raw(ctx: &DecodeCtx) -> Self {
        let offset = ctx.offset.get();
        let value = ctx.args[offset];
        ctx.advance_offset(1);
        value
    }
}

impl<'a> FixedDecode<'a> for u16 {
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn decode_raw(ctx: &DecodeCtx) -> Self {
        let offset = ctx.offset.get();
        let value = u16::from_le_bytes([ctx.args[offset], ctx.args[offset + 1]]);
        ctx.advance_offset(2);
        value
    }
}

impl<'a> FixedDecode<'a> for u32 {
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn decode_raw(ctx: &DecodeCtx) -> Self {
        let offset = ctx.offset.get();
        let value = u32::from_le_bytes([
            ctx.args[offset],
            ctx.args[offset + 1],
            ctx.args[offset + 2],
            ctx.args[offset + 3],
        ]);
        ctx.advance_offset(4);
        value
    }
}

impl<'a> FixedDecode<'a> for u64 {
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn decode_raw(ctx: &DecodeCtx) -> Self {
        let offset = ctx.offset.get();
        let value = u64::from_le_bytes([
            ctx.args[offset],
            ctx.args[offset + 1],
            ctx.args[offset + 2],
            ctx.args[offset + 3],
            ctx.args[offset + 4],
            ctx.args[offset + 5],
            ctx.args[offset + 6],
            ctx.args[offset + 7],
        ]);
        ctx.advance_offset(8);
        value
    }
}

impl<'a> FixedDecode<'a> for i64 {
    const ENCODED_SIZE: usize = size_of::<Self>();

    fn decode_raw(ctx: &DecodeCtx) -> Self {
        let offset = ctx.offset.get();
        let value = i64::from_le_bytes([
            ctx.args[offset],
            ctx.args[offset + 1],
            ctx.args[offset + 2],
            ctx.args[offset + 3],
            ctx.args[offset + 4],
            ctx.args[offset + 5],
            ctx.args[offset + 6],
            ctx.args[offset + 7],
        ]);
        ctx.advance_offset(8);
        value
    }
}
