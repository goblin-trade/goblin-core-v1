use crate::{
    input_processor::{DecodablePrimitive, DecodeCtx},
    quantities::{Exp, Quantity},
};

impl<E: Exp> DecodablePrimitive for Quantity<E> {
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self {
        Self::new(u64::decode_unchecked_no_advance(ctx))
    }
}
