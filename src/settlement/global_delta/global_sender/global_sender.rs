use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, SamePair},
        token::{
            token_marker::{hardcoded_erc20::HARDCODED_TOKENS, TokenMarker},
            CustomERC20, HardcodedERC20, Token, ETH,
        },
    },
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{global_delta::TokenDelta, local_delta::LocalDelta, CheckedOps, ConstZero},
    types::Triple,
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
}
