use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{
            token_marker::{
                CustomERC20Deltas, CustomERC20List, ETHDelta, HardcodedERC20Deltas, TokenMarker,
            },
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

    fn settle_leg<T>(
        &self,
        trader: &Address,
        custom_erc20_list: CustomERC20List,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
    {
        let sender_delta_list = T::get_leg(self);

        // run iter function now
        //
        // new strategy
        //
        // * keep trait monolitic. Split up later when API stabilizes
        let data_list = T::get_data_list(custom_erc20_list);

        // zipping the two lists together
        // We face problem with custom data list. Its length can be less than
        // length of its delta list

        // problems
        //
        // 1. custom list can have length more than MAX_CUSTOM_DELTAS
        // 2. We use fixed array instead of FixedMap which stores the number of active elements.
        // There is no concept of active or inactive item. We must check whether delta is non-zero
        // to save hostio calls.
        for (token_index, token_data) in T::token_index_data_iter(custom_erc20_list) {
            let delta = sender_delta_list[token_index];
            delta.settle(trader, &token_data, msg_transfers)?;
        }
        Ok(())
    }

    pub fn settle(
        &self,
        trader: &Address,
        custom_erc20_list: CustomERC20List,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError> {
        self.settle_leg::<ETH>(trader, custom_erc20_list, msg_transfers)?;
        self.settle_leg::<HardcodedERC20>(trader, custom_erc20_list, msg_transfers)?;
        self.settle_leg::<CustomERC20>(trader, custom_erc20_list, msg_transfers)?;

        Ok(())
    }
}
