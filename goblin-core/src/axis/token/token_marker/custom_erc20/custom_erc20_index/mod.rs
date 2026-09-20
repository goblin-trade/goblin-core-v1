use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;
use goblin_macros::ConstDefault;

#[derive(Clone, Copy, PartialEq, PartialOrd, ConstDefault, DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct CustomERC20Index {
    #[deku(bytes = "1", assert = "*inner <= Self::MAX_INNER")]
    pub inner: usize,
}

impl CustomERC20Index {
    /// 0b111 = 7 as it is decoded from 3 bits.
    pub const MAX_INNER: usize = 7;
}

impl From<usize> for CustomERC20Index {
    fn from(value: usize) -> Self {
        Self { inner: value }
    }
}
