use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    types::Tuple,
};

impl<T0, T1, K> Decodable for Tuple<T0, T1, K>
where
    T0: Decodable,
    T1: Decodable,
{
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        Ok(Tuple::new(T0::try_decode(ctx)?, T1::try_decode(ctx)?))
    }
}
