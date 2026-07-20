use crate::{
    axis::token::token_list::custom_erc20::CustomERC20List, goblin_error::GoblinError,
    input_processor::DecodeCtx, settlement::Delta, types::Address,
};

pub trait MarketCounts {
    /// Process legal combinations of market types
    fn process<'a>(
        &self,
        msg_sender: &Address,
        ctx: &DecodeCtx,
        custom_erc20_list: CustomERC20List<'a>,
        delta: &mut Delta,
    ) -> Result<(), GoblinError>;
}
