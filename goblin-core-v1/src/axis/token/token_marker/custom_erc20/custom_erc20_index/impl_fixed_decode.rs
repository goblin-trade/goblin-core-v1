use crate::{
    axis::token::token_marker::CustomERC20Index, goblin_error::GoblinError,
    input_processor::FixedDecode, require,
};

impl<'a> FixedDecode<'a> for CustomERC20Index {
    const ENCODED_SIZE: usize = 1;

    fn raw_fixed_decode(ctx: &'a crate::input_processor::DecodeCtx) -> Self {
        let index_raw = u8::raw_fixed_decode(ctx) as usize;
        CustomERC20Index(index_raw)
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(*self <= Self::MAX, GoblinError::InvalidPayload);
        Ok(())
    }
}
