use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{
            token_list::{
                custom_erc20::{CustomERC20Deltas, CustomERC20List},
                eth::ETHDelta,
                hardcoded_erc20::HardcodedERC20Deltas,
            },
            token_marker::{TokenData, TokenMarker},
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

    fn settle_leg<'a, T>(
        &'a self,
        trader: &Address,
        custom_erc20_list: CustomERC20List<'a>,
        msg_transfers: &MsgTransfers,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        &'a T::SenderDeltaList: IntoIterator<Item = &'a TokenDelta<T>>,
    {
        let data_list_iter = T::get_data_list(custom_erc20_list).into_iter();
        let sender_delta_list_iter = T::get_leg(self).into_iter();

        for (token_data, delta) in data_list_iter.zip(sender_delta_list_iter) {
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
