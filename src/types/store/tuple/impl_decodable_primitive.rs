use crate::{
    input_processor::{DecodablePrimitive, DecodeCtx},
    types::Tuple,
};

impl<T0, T1, K> DecodablePrimitive for Tuple<T0, T1, K>
where
    T0: DecodablePrimitive,
    T1: DecodablePrimitive,
{
    fn decode_unchecked_no_advance(ctx: &DecodeCtx) -> Self {
        Self::new(
            T0::decode_unchecked_no_advance(ctx),
            T1::decode_unchecked_no_advance(ctx),
        )
    }
}
