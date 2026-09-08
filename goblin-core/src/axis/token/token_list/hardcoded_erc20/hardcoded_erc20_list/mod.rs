pub mod chains;

pub use chains::*;

mod impl_into_iterator;

use crate::axis::token::{HardcodedERC20, token_marker::TokenData};

pub struct HardcodedERC20List<const N: usize> {
    pub inner: [TokenData<HardcodedERC20>; N],
}
