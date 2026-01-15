use crate::{
    goblin_error::GoblinError,
    token::{CustomERC20Data, ERC20Index, ERC20Marker},
};

#[derive(Clone, Copy, PartialEq)]
pub struct CustomERC20;

impl ERC20Marker for CustomERC20 {
    type Data = CustomERC20Data;

    fn get_data(
        erc20_index: ERC20Index<Self>,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<&Self::Data, GoblinError> {
        custom_erc20_list
            .get(erc20_index.inner as usize)
            .ok_or(GoblinError::InvalidCustomTokenIndex)
    }
}
