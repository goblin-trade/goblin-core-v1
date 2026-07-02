use core::marker::PhantomData;

use crate::{
    axis::token::token_marker::{custom_erc20::custom_erc20_list::CustomERC20List, TokenMarker},
    goblin_error::GoblinError,
    settlement::ConstZero,
};

pub trait TokenIndex<T: TokenMarker> {
    fn get_address(
        &self,
        custom_erc20_list: CustomERC20List,
    ) -> Result<T::TokenAddress, GoblinError>;
}
