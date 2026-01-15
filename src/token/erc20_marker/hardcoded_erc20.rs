use crate::{
    goblin_error::GoblinError,
    token::{CustomERC20Data, ERC20Marker, HardcodedERC20Data, ERC20Index, HARDCODED_TOKENS},
};

#[derive(Clone, Copy, PartialEq)]
pub struct HardcodedERC20;

impl ERC20Marker for HardcodedERC20 {
    type Data = HardcodedERC20Data;

    fn get_data(
        token_index: ERC20Index<Self>,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<&Self::Data, GoblinError> {
        HARDCODED_TOKENS
            .get(token_index.inner as usize)
            .ok_or(GoblinError::InvalidHardcodedTokenIndex)
    }
}
