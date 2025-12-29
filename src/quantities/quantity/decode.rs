use crate::{
    input_processor::{DecodeCtx, DecodablePrimitive},
    quantities::{Exp, Quantity},
};

impl<'a, D: Exp> DecodablePrimitive<'a> for Quantity<D> {
    fn decode_unchecked_no_advance(ctx: &'a DecodeCtx<'a>) -> Self {
        Self::new(u64::decode_unchecked_no_advance(ctx))
    }
}
