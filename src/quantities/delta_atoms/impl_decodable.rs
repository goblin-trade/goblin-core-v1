use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    quantities::DeltaAtoms,
};

impl Decodable for DeltaAtoms {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        i64::try_decode(ctx).map(DeltaAtoms::new)
    }
}
