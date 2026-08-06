use crate::{
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    require,
};

pub trait CheckedFixedDecode<'a>: FixedDecode<'a> + PartialOrd {
    const MAX: Self;

    fn try_checked_decode(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        let value = Self::try_fixed_decode(ctx)?;
        require!(value <= Self::MAX, GoblinError::InvalidPayload);

        Ok(value)
    }
}
