use core::ops::Index;

use crate::axis::token::{
    token_index::{CustomERC20Index, CustomERC20List, TokenData},
    CustomERC20,
};

impl<'a> Index<CustomERC20Index> for CustomERC20List<'a> {
    type Output = TokenData<CustomERC20>;

    fn index(&self, index: CustomERC20Index) -> &Self::Output {
        &self.inner[index.0]
    }
}
