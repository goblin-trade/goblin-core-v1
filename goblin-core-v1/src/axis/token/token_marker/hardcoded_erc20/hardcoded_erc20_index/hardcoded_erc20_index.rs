use goblin_macros::ConstZero;

use crate::axis::token::token_list::hardcoded_erc20::HARDCODED_ERC20_COUNT;

#[derive(Clone, Copy, PartialEq, PartialOrd, ConstZero)]
pub struct HardcodedERC20Index(pub usize);

impl HardcodedERC20Index {
    pub const MAX: Self = Self(HARDCODED_ERC20_COUNT - 1);
}

impl From<usize> for HardcodedERC20Index {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
