use crate::{goblin_error::GoblinError, input_processor::ArgsBuffer};

pub trait Decodable<T>: Sized {
    fn decode(payload: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<T, GoblinError>;
}
