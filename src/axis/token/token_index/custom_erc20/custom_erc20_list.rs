use crate::{
    axis::token::{
        token_index::{CustomERC20Index, TokenData},
        CustomERC20,
    },
    goblin_error::GoblinError,
    types::Address,
};

#[derive(Clone, Copy)]
pub struct CustomERC20List<'a> {
    pub inner: &'a [TokenData<CustomERC20>],
}

// TODO Index trait instead of custom function
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
}
