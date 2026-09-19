use deku::DekuRead;
use goblin_macros::{ConstDefault, fixed_codec};

mod impl_index;

use crate::{axis::token::token_list::HARDCODED_ERC20_COUNT, goblin_error::GoblinError, require};

#[fixed_codec(validate = Self::check)]
#[derive(Clone, Copy, PartialEq, PartialOrd, ConstDefault, DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct HardcodedERC20Index {
    #[codec(wire = u8)]
    #[deku(bytes = "1", assert = "*inner <= Self::MAX_INNER")]
    pub inner: usize,
}

impl HardcodedERC20Index {
    pub const MAX_INNER: usize = HARDCODED_ERC20_COUNT - 1;

    pub const fn new(inner: usize) -> Self {
        Self { inner }
    }

    /// Range check kept out of the generated codec via `validate = Self::check`.
    fn check(&self) -> Result<(), GoblinError> {
        require!(self.inner <= Self::MAX_INNER, GoblinError::InvalidPayload);
        Ok(())
    }
}

impl From<usize> for HardcodedERC20Index {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
