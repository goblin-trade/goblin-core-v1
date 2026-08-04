use crate::{input_processor::FixedDecode, types::Tuple};

impl<'a, T0, T1, K> FixedDecode<'a> for Tuple<T0, T1, K>
where
    T0: FixedDecode<'a>,
    T1: FixedDecode<'a>,
{
    const ENCODED_SIZE: usize = T0::ENCODED_SIZE + T1::ENCODED_SIZE;

    fn decode_raw(ctx: &'a crate::input_processor::DecodeCtx) -> Self {
        Self::new(T0::decode_raw(ctx), T1::decode_raw(ctx))
    }
}
