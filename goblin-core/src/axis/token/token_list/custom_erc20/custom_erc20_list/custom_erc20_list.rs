use crate::axis::token::{token_marker::TokenData, CustomERC20};

#[derive(Clone, Copy)]
pub struct CustomERC20List<'a> {
    pub inner: &'a [TokenData<CustomERC20>],
}
