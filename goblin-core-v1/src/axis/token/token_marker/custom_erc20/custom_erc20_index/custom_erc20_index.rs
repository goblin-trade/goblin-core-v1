use crate::settlement::ConstZero;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct CustomERC20Index(pub usize);

impl CustomERC20Index {
    /// 0b111 = 7 as it is decoded from 3 bits.
    pub const MAX_COUNT: usize = 7;

    pub const MAX: Self = Self(Self::MAX_COUNT - 1);
}

impl ConstZero for CustomERC20Index {
    const ZEROED: Self = Self(0);
}

impl From<usize> for CustomERC20Index {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
