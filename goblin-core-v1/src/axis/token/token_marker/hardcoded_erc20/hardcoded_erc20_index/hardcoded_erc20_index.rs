use goblin_macros::ConstDefault;

use crate::axis::HARDCODED_ERC20_COUNT;

#[derive(Clone, Copy, PartialEq, PartialOrd, ConstDefault)]
pub struct HardcodedERC20Index(pub usize);

impl HardcodedERC20Index {
    pub const MAX: Self = Self(HARDCODED_ERC20_COUNT - 1);
}

impl From<usize> for HardcodedERC20Index {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
