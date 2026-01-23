use crate::{
    input_processor::{DecodablePrimitive, DecodeCtx},
    quantities::{Exp, Quantity},
};

impl<D: Exp> DecodablePrimitive for Quantity<D> {
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self {
        Self::new(u64::decode_unchecked_no_advance(ctx))
    }
}
