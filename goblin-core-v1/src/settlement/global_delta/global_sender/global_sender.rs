use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_to_token::LegToToken, SamePair},
        market::{token_pair::TokenPair, TokenIndexPair},
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
    quantities::UnsidedAtomsPerLot,
    settlement::{
        global_delta::{FromLocalDelta, TokenDelta},
        local_delta::LocalDelta,
        CheckedOps, ConstDefault,
    },
    types::{Address, StoreReader, Triple},
};

pub type GlobalSender = Triple<ETHDelta, HardcodedERC20Deltas, CustomERC20Deltas, Token>;

impl GlobalSender {
    pub fn commit_leg<TP, In>(
        &mut self,
        local_delta: &LocalDelta,
        token_index_pair: &TokenIndexPair<TP>,
        atoms_per_lot_pair: &SamePair<UnsidedAtomsPerLot>,
    ) -> Result<(), GoblinError>
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>,
    {
        let delta_atoms_per_lot_pair = atoms_per_lot_pair.try_into()?;
        let new_delta = TokenDelta::from_local_delta::<In>(local_delta, &delta_atoms_per_lot_pair);

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
            if *delta != TokenDelta::DEFAULT {
                delta.settle(trader, &token_data, msg_transfers)?;
            }
        }

        Ok(())
    }
}
