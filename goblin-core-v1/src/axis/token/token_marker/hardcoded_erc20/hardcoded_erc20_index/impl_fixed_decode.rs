use crate::{
    axis::token::token_marker::HardcodedERC20Index,
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    require,
};

impl<'a> FixedDecode<'a> for HardcodedERC20Index {
    const ENCODED_SIZE: usize = 1;

    fn raw_fixed_decode(ctx: &'a DecodeCtx) -> Self {
        let index_raw = u8::raw_fixed_decode(ctx) as usize;
        Self(index_raw)
    }

    fn validate(&self) -> Result<(), GoblinError> {
        require!(*self <= Self::MAX, GoblinError::InvalidPayload);
        Ok(())
    }
}
