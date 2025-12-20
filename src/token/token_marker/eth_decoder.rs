use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    token::ETH,
};

impl Decodable<()> for ETH {
    fn decode(args: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<(), GoblinError> {
        Ok(())
    }
}
