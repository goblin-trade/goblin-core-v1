use crate::input_processor::DecodeCtx;

pub trait VariableDecode {
    type Flags;

    fn decode(ctx: &DecodeCtx, flags: Self::Flags) -> Self;
}
