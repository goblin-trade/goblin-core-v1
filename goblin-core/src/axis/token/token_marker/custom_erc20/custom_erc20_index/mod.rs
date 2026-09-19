use deku::DekuRead;
use goblin_macros::{ConstDefault, fixed_codec};

use crate::{goblin_error::GoblinError, require};

#[fixed_codec(validate = Self::check)]
#[derive(Clone, Copy, PartialEq, PartialOrd, ConstDefault, DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct CustomERC20Index {
    #[codec(wire = u8)]
    pub inner: usize,
}

impl CustomERC20Index {
    /// 0b111 = 7 as it is decoded from 3 bits.
    pub const MAX_COUNT: usize = 7;

    pub const MAX: Self = Self {
        inner: Self::MAX_COUNT - 1,
    };

    /// Range check kept out of the generated codec via `validate = Self::check`.
    fn check(&self) -> Result<(), GoblinError> {
        require!(self.inner <= Self::MAX.inner, GoblinError::InvalidPayload);
        Ok(())
    }
}

impl From<usize> for CustomERC20Index {
    fn from(value: usize) -> Self {
        Self { inner: value }
    }
}
