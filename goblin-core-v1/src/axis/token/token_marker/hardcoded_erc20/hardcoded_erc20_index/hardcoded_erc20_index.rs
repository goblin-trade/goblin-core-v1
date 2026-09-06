use goblin_macros::ConstDefault;

use crate::axis::HARDCODED_ERC20_COUNT;

#[derive(Clone, Copy, PartialEq, PartialOrd, ConstDefault)]
pub struct HardcodedERC20Index {
    pub inner: usize,
}

impl HardcodedERC20Index {
    pub const MAX: Self = Self::new(HARDCODED_ERC20_COUNT - 1);

    pub const fn new(inner: usize) -> Self {
        Self { inner }
    }
}

impl From<usize> for HardcodedERC20Index {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}
