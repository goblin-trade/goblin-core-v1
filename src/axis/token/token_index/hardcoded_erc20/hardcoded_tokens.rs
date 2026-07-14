use crate::axis::token::{token_index::TokenData, HardcodedERC20};

pub struct HardcodedTokens<const N: usize> {
    pub inner: [TokenData<HardcodedERC20>; N],
}
