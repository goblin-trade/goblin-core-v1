use crate::input_processor::DecodeCtx;

/// Decode primitive types from little endian bytes without bound checks
///
/// These functions are used in Decodable::try_decode() which applies bounds checks.
/// We can optimize batch decoding by skipping per variable checks, instead doing
/// it together for multiple variables.
pub trait DecodablePrimitive: Sized {
    /// Decode without bound checks without offset advancement
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self;
}

impl DecodablePrimitive for u8 {
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self {
        let offset = ctx.offset.get();
        ctx.args[offset]
    }
}

impl DecodablePrimitive for u32 {
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self {
        let offset = ctx.offset.get();
        u32::from_le_bytes([
            ctx.args[offset],
            ctx.args[offset + 1],
            ctx.args[offset + 2],
            ctx.args[offset + 3],
        ])
    }
}

impl DecodablePrimitive for u16 {
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self {
        let offset = ctx.offset.get();
        u16::from_le_bytes([ctx.args[offset], ctx.args[offset + 1]])
    }
}

impl DecodablePrimitive for u64 {
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self {
        let offset = ctx.offset.get();
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

impl DecodablePrimitive for i64 {
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self {
        let offset = ctx.offset.get();
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
