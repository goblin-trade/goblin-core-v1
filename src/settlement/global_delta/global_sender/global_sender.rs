use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_to_token::LegToToken, SamePair},
        market::TokenIndexPair,
        token::{
            token_list::{
                custom_erc20::CustomERC20Deltas, eth::ETHDelta,
                hardcoded_erc20::HardcodedERC20Deltas,
            },
            token_marker::TokenMarker,
            token_quantity::TokenQuantity,
            token_reader::TokenDataTriple,
            Token,
        },
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{
        global_delta::{FromLocalDelta, TokenDelta},
        local_delta::LocalDelta,
        CheckedOps, ConstZero,
    },
    types::{Address, StoreReader, Triple},
};

pub type GlobalSender = Triple<ETHDelta, HardcodedERC20Deltas, CustomERC20Deltas, Token>;

impl GlobalSender {
    pub fn commit_side<B, Q, In>(
        &mut self,
        token_index_pair: &TokenIndexPair<B, Q>,
        atoms_per_lot_pair: &SamePair<UnsidedDeltaAtomsPerLot>,
        local_delta: &LocalDelta,
    ) -> Result<(), GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher
            + LegToToken<B, Q>
            + StoreReader<TokenIndexPair<B, Q>, Result = <In::Selected as TokenQuantity>::TokenIndex>,
    {
        let new_delta = TokenDelta::from_local_delta::<In>(atoms_per_lot_pair, local_delta);

        let token_index = In::get(token_index_pair);
        let deltas_list = In::Selected::get_leg_mut(self);
        let delta_store = &mut deltas_list[token_index];

        *delta_store = delta_store
            .checked_add(new_delta)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }

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
            if *delta != TokenDelta::ZEROED {
                delta.settle(trader, &token_data, msg_transfers)?;
            }
        }

        Ok(())
    }
}
