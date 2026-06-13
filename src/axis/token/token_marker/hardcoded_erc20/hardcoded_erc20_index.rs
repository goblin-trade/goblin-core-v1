use crate::settlement::ConstZero;

#[derive(Clone, Copy, PartialEq)]
pub struct HardcodedERC20Index(pub usize);

impl ConstZero for HardcodedERC20Index {
    const ZEROED: Self = Self(0);
}
