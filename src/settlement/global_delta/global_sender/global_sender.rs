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
        update::{Decrease, Increase, UpdateERC20},
    },
    goblin_error::GoblinError,
    hostio::erc20_hostio,
    quantities::{
        IntoAbs, RawAtoms, TryIntoUnsidedDelta, UnsidedAtoms, UnsidedDeltaAtoms,
        UnsidedDeltaAtomsPerLot,
    },
    settlement::{
        global_delta::{transfer_deposit, TokenDelta},
        local_delta::LocalDelta,
        CheckedOps, ConstZero,
    },
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
        trader: &Address,
        custom_erc20_list: CustomERC20List,
    ) -> Result<(), GoblinError> {
        let custom_deltas = CustomERC20::get(self);

        for (token_index, token_address) in custom_erc20_list.iter() {
            let delta = custom_deltas[token_index.0];

            let store_hash = StorePreimage::<CustomERC20> {
                trader: *trader,
                token: token_address,
            }
            .hash();

            let mut store = store_hash.load();
            let atoms_free_delta = UnsidedDeltaAtoms::try_from(store.atoms_free)?
                + delta.deposit
                + delta.make
                + delta.take;

            let atoms_locked_delta = UnsidedDeltaAtoms::try_from(store.atoms_locked)? - delta.make;

            // Return error if free atoms > 0 or if we overflow
            store.atoms_free = UnsidedAtoms::try_from(atoms_free_delta)?;
            store.atoms_locked = UnsidedAtoms::try_from(atoms_locked_delta)?;

            store_hash.store(&store);

            if delta.deposit == UnsidedDeltaAtoms::ZEROED {
                continue;
            }

            let deposit_abs = delta.deposit.abs();
            let decimals = erc20_hostio::decimals(&token_address)?;

            if delta.deposit > UnsidedDeltaAtoms::ZEROED {
                transfer_deposit::<Increase>(deposit_abs, decimals, &token_address, trader)?;
            } else {
                transfer_deposit::<Decrease>(deposit_abs, decimals, &token_address, trader)?;
            }
        }
        Ok(())
    }
}
