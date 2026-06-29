use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{
            token_marker::{
                custom_erc20::{
                    custom_erc20_data::CustomERC20Data, custom_erc20_list::CustomERC20List,
                },
                hardcoded_erc20::HARDCODED_TOKENS,
                TokenMarker,
            },
            CustomERC20, HardcodedERC20, Token, ETH,
        },
        update::Increase,
    },
    goblin_error::GoblinError,
    quantities::{TryIntoUnsidedDelta, UnsidedDeltaAtomsPerLot},
    settlement::{global_delta::TokenDelta, local_delta::LocalDelta, CheckedOps, ConstZero},
    state::{Preimage, StorePreimage},
    types::{Address, StoreReader, Triple},
};

pub const MAX_HARDCODED_DELTAS: usize = HARDCODED_TOKENS.len();
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

    fn settle(
        &self,
        trader: Address,
        custom_erc20_list: CustomERC20List,
    ) -> Result<(), GoblinError> {
        let custom_deltas = CustomERC20::get(self);

        for (token_index, token_address) in custom_erc20_list.iter() {
            let delta = custom_deltas[token_index.0];

            let store_hash = StorePreimage::<CustomERC20> {
                trader,
                token: token_address,
            }
            .hash();

            let store = store_hash.load();

            // let free_delta = store.atoms_free.try_into_unsided_delta::<Increase>()?;
        }
        Ok(())
    }
}
