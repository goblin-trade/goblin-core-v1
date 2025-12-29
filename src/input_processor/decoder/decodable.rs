use crate::{goblin_error::GoblinError, input_processor::ArgsBuffer};

pub trait Decodable: Sized {
    fn decode(args: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<Self, GoblinError>;
}
