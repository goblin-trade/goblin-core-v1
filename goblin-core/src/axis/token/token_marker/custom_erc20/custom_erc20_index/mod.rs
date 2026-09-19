use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;
use goblin_macros::{ConstDefault, fixed_codec};

use crate::{goblin_error::GoblinError, require};

#[fixed_codec(validate = Self::check)]
#[derive(Clone, Copy, PartialEq, PartialOrd, ConstDefault, DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct CustomERC20Index {
    #[codec(wire = u8)]
    #[deku(bytes = "1", assert = "*inner <= Self::MAX_INNER")]
    pub inner: usize,
}

impl CustomERC20Index {
    /// 0b111 = 7 as it is decoded from 3 bits.
    pub const MAX_INNER: usize = 7;

    /// Range check kept out of the generated codec via `validate = Self::check`.
    fn check(&self) -> Result<(), GoblinError> {
        require!(self.inner <= Self::MAX_INNER, GoblinError::InvalidPayload);
        Ok(())
    }
}

impl From<usize> for CustomERC20Index {
    fn from(value: usize) -> Self {
        Self { inner: value }
    }
}
