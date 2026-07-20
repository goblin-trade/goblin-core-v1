use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{
            token_list::{
                custom_erc20::CustomERC20Deltas, eth::ETHDelta,
                hardcoded_erc20::HardcodedERC20Deltas,
            },
            token_marker::TokenMarker,
            token_reader::TokenDataTriple,
            CustomERC20, HardcodedERC20, Token, ETH,
        },
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{global_delta::TokenDelta, local_delta::LocalDelta, CheckedOps, ConstZero},
    types::{Address, Triple},
};

pub type GlobalSender = Triple<ETHDelta, HardcodedERC20Deltas, CustomERC20Deltas, Token>;

impl GlobalSender {
    pub fn commit_side<T, In>(
        &mut self,
        token_index: T::TokenIndex,
        atoms_per_lot_pair: &SamePair<UnsidedDeltaAtomsPerLot>,
        local_delta: &LocalDelta,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        In: LegMatcher,
    {
        let new_delta = TokenDelta::<T>::from_local_delta::<In>(atoms_per_lot_pair, local_delta);

        let deltas_list = T::get_leg_mut(self);
        let delta_store = &mut deltas_list[token_index];

        *delta_store = delta_store
            .checked_add(new_delta)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }

    pub fn settle(
        &self,
        trader: &Address,
        token_data_triple: &TokenDataTriple,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError> {
        self.settle_leg::<ETH>(trader, token_data_triple, msg_transfers)?;
        self.settle_leg::<HardcodedERC20>(trader, token_data_triple, msg_transfers)?;
        self.settle_leg::<CustomERC20>(trader, token_data_triple, msg_transfers)?;

        Ok(())
    }

    fn settle_leg<'a, T>(
        &'a self,
        trader: &Address,
        token_data_triple: &TokenDataTriple<'a>,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        &'a T::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<T>>,
    {
        let data_list_iter = T::get_with_lifetime(&token_data_triple).into_iter();
        let sender_delta_list_iter = T::get_leg(self).into_iter();

        for (token_data, delta) in data_list_iter.zip(sender_delta_list_iter) {
            if *delta != TokenDelta::ZEROED {
                delta.settle(trader, &token_data, msg_transfers)?;
            }
        }

        Ok(())
    }
}
