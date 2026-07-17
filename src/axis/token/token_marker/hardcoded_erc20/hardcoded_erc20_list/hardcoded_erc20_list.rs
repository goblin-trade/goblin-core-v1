use crate::axis::token::{token_marker::TokenData, HardcodedERC20};

pub struct HardcodedERC20List<const N: usize> {
    pub inner: [TokenData<HardcodedERC20>; N],
}
