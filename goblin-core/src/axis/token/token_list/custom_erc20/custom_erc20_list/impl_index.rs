use core::ops::Index;

use crate::axis::token::{
    CustomERC20,
    token_list::custom_erc20::CustomERC20List,
    token_marker::{CustomERC20Index, TokenData},
};

const impl<'a> Index<CustomERC20Index> for CustomERC20List<'a> {
    type Output = TokenData<CustomERC20>;

    fn index(&self, index: CustomERC20Index) -> &Self::Output {
        &self.inner[index.0]
    }
}
