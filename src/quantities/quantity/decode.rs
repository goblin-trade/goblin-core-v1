use crate::{
    input_processor::{DecodablePrimitive, DecodeCtx},
    quantities::{Exp, Quantity},
};

impl<E> DecodablePrimitive for Quantity<E, u64>
where
    E: Exp,
{
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self {
        Self::new(u64::decode_unchecked_no_advance(ctx))
    }
}
