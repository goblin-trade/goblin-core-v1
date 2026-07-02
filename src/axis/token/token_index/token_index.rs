use crate::{
    axis::token::token_index::CustomERC20List, goblin_error::GoblinError,
    input_processor::Decodable, settlement::ConstZero,
};

pub trait TokenIndex: Clone + Copy + Decodable + ConstZero + PartialEq {
    type TokenAddress: Clone + Copy + Sized + Default;

    fn get_address(
        &self,
        custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError>;
}
