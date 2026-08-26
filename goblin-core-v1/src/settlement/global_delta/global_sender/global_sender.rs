use crate::{
    axis::token::{
        token_list::{
            custom_erc20::CustomERC20Deltas, eth::ETHDelta, hardcoded_erc20::HardcodedERC20Deltas,
        },
        token_marker::TokenMarker,
        token_reader::TokenDataTriple,
        Token,
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    settlement::{global_delta::TokenDelta, ConstDefault},
    types::{Address, Triple},
};

pub type GlobalSender = Triple<ETHDelta, HardcodedERC20Deltas, CustomERC20Deltas, Token>;

impl GlobalSender {
    pub fn settle_leg<'a, T>(
        &'a self,
        trader: &Address,
        token_data_triple: &TokenDataTriple<'a>,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        &'a T::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<T>>,
    {
        let data_list_iter = T::get_lifetimed(&token_data_triple).into_iter();
        let sender_delta_list_iter = T::get_leg(self).into_iter();

        for (token_data, delta) in data_list_iter.zip(sender_delta_list_iter) {
            if *delta != TokenDelta::DEFAULT {
                delta.settle(trader, &token_data, msg_transfers)?;
            }
        }

        Ok(())
    }
}
