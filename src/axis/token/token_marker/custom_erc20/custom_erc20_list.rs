use crate::{
    axis::token::token_marker::custom_erc20::{
        custom_erc20_data::CustomERC20Data, custom_erc20_index::CustomERC20Index,
    },
    goblin_error::GoblinError,
    types::Address,
};

#[derive(Clone, Copy)]
pub struct CustomERC20List<'a> {
    pub inner: &'a [CustomERC20Data],
}

impl<'a> CustomERC20List<'a> {
    pub fn token_index_to_address(
        &self,
        token_index: CustomERC20Index,
    ) -> Result<Address, GoblinError> {
        let data = self
            .inner
            .get(token_index.0)
            .ok_or(GoblinError::InvalidCustomTokenIndex)?;
        Ok(data.address)
    }

    pub fn index_iter(&self) -> impl Iterator<Item = CustomERC20Index> {
        (0..self.inner.len()).map(CustomERC20Index::from)
    }

    pub fn iter(&self) -> impl Iterator<Item = (CustomERC20Index, Address)> + 'a {
        self.inner
            .iter()
            .enumerate()
            .map(|(i, data)| (CustomERC20Index::from(i), data.address))
    }
}
