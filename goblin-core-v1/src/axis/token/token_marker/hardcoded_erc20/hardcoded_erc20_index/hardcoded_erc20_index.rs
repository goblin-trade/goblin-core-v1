use crate::{
    axis::token::token_list::hardcoded_erc20::HARDCODED_ERC20_COUNT, settlement::ConstZero,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct HardcodedERC20Index(pub usize);

impl HardcodedERC20Index {
    pub const MAX: Self = Self(HARDCODED_ERC20_COUNT - 1);
}

impl ConstZero for HardcodedERC20Index {
    const ZEROED: Self = Self(0);
}

impl From<usize> for HardcodedERC20Index {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
