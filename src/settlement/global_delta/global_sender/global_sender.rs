use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{
            token_global_transfer::ETHTransfers,
            token_index::{CustomERC20List, TokenData, HARDCODED_TOKENS, MAX_HARDCODED_DELTAS},
            token_marker::TokenMarker,
            CustomERC20, CustomERC20Stub, HardcodedERC20, HardcodedERC20Stub, Token, ETH,
        },
    },
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{global_delta::TokenDelta, local_delta::LocalDelta, CheckedOps, ConstZero},
    types::{Address, StoreReader, Triple},
};

pub const MAX_CUSTOM_DELTAS: usize = 8;

pub type GlobalSender = Triple<
    TokenDelta<ETH>,
    [TokenDelta<HardcodedERC20>; MAX_HARDCODED_DELTAS],
    [TokenDelta<CustomERC20>; MAX_CUSTOM_DELTAS],
    Token,
>;

impl ConstZero for GlobalSender {
    const ZEROED: Self = Self::new(
        TokenDelta::ZEROED,
        [TokenDelta::ZEROED; MAX_HARDCODED_DELTAS],
        [TokenDelta::ZEROED; MAX_CUSTOM_DELTAS],
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
        let delta = TokenDelta::from_local_delta::<In>(atoms_per_lot_pair, local_delta);

        let delta_store = T::get_global_token_delta(token_index, self);
        *delta_store = delta_store
            .checked_add(delta)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }

    pub fn settle(
        &self,
        trader: &Address,
        custom_erc20_list: CustomERC20List,
        eth_transfers: ETHTransfers,
    ) -> Result<(), GoblinError> {
        let eth_delta = ETH::get_leg(self);
        eth_delta.settle(trader, TokenData::ETH_STUB_PAIR, eth_transfers)?;

        let hardcoded_deltas = HardcodedERC20::get_leg(self);
        for (token_index, token_data) in HARDCODED_TOKENS.typed_iter() {
            //  TODO use core::ops::Index trait
            let hardcoded_erc20_delta = hardcoded_deltas[token_index.0];

            hardcoded_erc20_delta.settle(trader, (token_index, token_data), HardcodedERC20Stub)?;
        }

        let custom_deltas = CustomERC20::get_leg(self);
        for (token_index, token_data) in custom_erc20_list.typed_iter() {
            let custom_erc20_delta = custom_deltas[token_index.0];
            custom_erc20_delta.settle(trader, (token_index, token_data), CustomERC20Stub)?;
        }

        Ok(())
    }
}
