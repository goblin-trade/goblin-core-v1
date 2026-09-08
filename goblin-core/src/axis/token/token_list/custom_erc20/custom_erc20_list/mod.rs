mod impl_index;
mod impl_into_iterator;
mod impl_variable_decode;

use crate::axis::token::{CustomERC20, token_marker::TokenData};

#[derive(Clone, Copy)]
pub struct CustomERC20List<'a> {
    pub inner: &'a [TokenData<CustomERC20>],
}
