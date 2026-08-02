use crate::settlement::ConstZero;

#[derive(Clone, Copy, PartialEq)]
pub struct CustomERC20Index(pub usize);

impl ConstZero for CustomERC20Index {
    const ZEROED: Self = Self(0);
}

impl From<usize> for CustomERC20Index {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
