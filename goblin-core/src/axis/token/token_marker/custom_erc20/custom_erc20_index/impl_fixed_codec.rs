use crate::{
    axis::token::token_marker::CustomERC20Index,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, ArgsWriter, FixedCodec},
    require,
};

impl FixedCodec for CustomERC20Index {
    const ENCODED_SIZE: usize = 1;

    fn raw_fixed_decode(reader: &ArgsReader) -> Self {
        let index_raw = u8::raw_fixed_decode(reader) as usize;
        CustomERC20Index(index_raw)
    }

    fn raw_fixed_encode(&self, writer: &mut ArgsWriter) {
        (self.0 as u8).raw_fixed_encode(writer);
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(*self <= Self::MAX, GoblinError::InvalidPayload);
        Ok(())
    }
}
