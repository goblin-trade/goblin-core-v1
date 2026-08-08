use goblin_macros::ConstZero;

#[derive(Clone, Copy, PartialEq, PartialOrd, ConstZero)]
pub struct CustomERC20Index(pub usize);

impl CustomERC20Index {
    /// 0b111 = 7 as it is decoded from 3 bits.
    pub const MAX_COUNT: usize = 7;

    pub const MAX: Self = Self(Self::MAX_COUNT - 1);
}

impl From<usize> for CustomERC20Index {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
