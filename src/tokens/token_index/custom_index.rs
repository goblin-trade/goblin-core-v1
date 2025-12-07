use crate::tokens::{CustomToken, TokenIndex};

pub type CustomIndex = TokenIndex<CustomToken>;

impl CustomIndex {
    pub fn get_token<'a>(&self, custom_erc20_list: &'a [CustomToken]) -> Option<&'a CustomToken> {
        custom_erc20_list.get(self.inner as usize)
    }
}
