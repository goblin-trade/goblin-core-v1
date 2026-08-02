use crate::input_processor::{DecodableV2, DecodeCtx};

impl DecodableV2 for u8 {
    const SIZE: usize = 1;
    fn decode_unchecked(ctx: &DecodeCtx, offset: usize) -> Self {
        ctx.args[offset]
    }
}

impl DecodableV2 for u16 {
    const SIZE: usize = 2;
    fn decode_unchecked(ctx: &DecodeCtx, offset: usize) -> Self {
        u16::from_le_bytes([ctx.args[offset], ctx.args[offset + 1]])
    }
}

impl DecodableV2 for u32 {
    const SIZE: usize = 4;
    fn decode_unchecked(ctx: &DecodeCtx, offset: usize) -> Self {
        u32::from_le_bytes([
            ctx.args[offset],
            ctx.args[offset + 1],
            ctx.args[offset + 2],
            ctx.args[offset + 3],
        ])
    }
}

impl DecodableV2 for u64 {
    const SIZE: usize = 8;
    fn decode_unchecked(ctx: &DecodeCtx, offset: usize) -> Self {
        u64::from_le_bytes([
            ctx.args[offset],
            ctx.args[offset + 1],
            ctx.args[offset + 2],
            ctx.args[offset + 3],
            ctx.args[offset + 4],
            ctx.args[offset + 5],
            ctx.args[offset + 6],
            ctx.args[offset + 7],
        ])
    }
}

impl DecodableV2 for i64 {
    const SIZE: usize = 8;
    fn decode_unchecked(ctx: &DecodeCtx, offset: usize) -> Self {
        i64::from_le_bytes([
            ctx.args[offset],
            ctx.args[offset + 1],
            ctx.args[offset + 2],
            ctx.args[offset + 3],
            ctx.args[offset + 4],
            ctx.args[offset + 5],
            ctx.args[offset + 6],
            ctx.args[offset + 7],
        ])
    }
}
