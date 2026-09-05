use crate::{
    axis::HardcodedERC20Index,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    require,
};

impl<'a> FixedDecode<'a> for HardcodedERC20Index {
    const ENCODED_SIZE: usize = 1;

    fn raw_fixed_decode(reader: &'a ArgsReader) -> Self {
        let index_raw = u8::raw_fixed_decode(reader) as usize;
        Self(index_raw)
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(*self <= Self::MAX, GoblinError::InvalidPayload);
        Ok(())
    }
}
