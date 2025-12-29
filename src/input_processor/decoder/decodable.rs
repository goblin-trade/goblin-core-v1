use crate::{goblin_error::GoblinError, input_processor::DecodeCtx};

pub trait Decodable<'a>: Sized {
    fn decode(ctx: &'a DecodeCtx<'a>) -> Result<Self, GoblinError>;
}
