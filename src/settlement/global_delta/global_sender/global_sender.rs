use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{
            token_index::{CustomERC20List, TokenData, HARDCODED_TOKENS},
            token_marker::TokenMarker,
            CustomERC20, HardcodedERC20, Token, ETH,
        },
    },
    goblin_error::GoblinError,
    input_processor::MsgTransfers,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{
        global_delta::{CustomERC20Deltas, ETHDelta, HardcodedERC20Deltas, TokenDelta},
        local_delta::LocalDelta,
        CheckedOps, ConstZero,
    },
    types::{Address, StoreReader, Triple},
};

pub type GlobalSender = Triple<ETHDelta, HardcodedERC20Deltas, CustomERC20Deltas, Token>;

impl ConstZero for GlobalSender {
    const ZEROED: Self = Self::new(
        ETHDelta::ZEROED,
        HardcodedERC20Deltas::ZEROED,
        CustomERC20Deltas::ZEROED,
    );
}

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
        custom_erc20_list: CustomERC20List,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError> {
        let eth_delta = ETH::get_leg(self).inner;
        eth_delta.settle(trader, TokenData::ETH_STUB_PAIR, msg_transfers)?;

        let hardcoded_deltas = HardcodedERC20::get_leg(self);
        for (token_index, token_data) in HARDCODED_TOKENS.typed_iter() {
            let hardcoded_erc20_delta = hardcoded_deltas[token_index];
            hardcoded_erc20_delta.settle(trader, (token_index, token_data), msg_transfers)?;
        }

        let custom_deltas = CustomERC20::get_leg(self);
        for (token_index, token_data) in custom_erc20_list.typed_iter() {
            let custom_erc20_delta = custom_deltas[token_index];
            custom_erc20_delta.settle(trader, (token_index, token_data), msg_transfers)?;
        }

        Ok(())
    }
}
