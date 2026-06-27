use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        token::{
            token_marker::{hardcoded_erc20::HARDCODED_TOKENS, TokenMarker},
            CustomERC20, HardcodedERC20, Token, ETH,
        },
    },
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{
        global_delta_v3::TokenDeltaV3, local_delta_v3::LocalDeltaV3, CheckedOps, ConstZero,
    },
    types::Triple,
};

pub const MAX_HARDCODED_DELTAS_V3: usize = HARDCODED_TOKENS.len();
pub const MAX_CUSTOM_DELTAS_V3: usize = 8;

pub type GlobalSender = Triple<
    TokenDeltaV3<ETH>,
    [TokenDeltaV3<HardcodedERC20>; MAX_HARDCODED_DELTAS_V3],
    [TokenDeltaV3<CustomERC20>; MAX_CUSTOM_DELTAS_V3],
    Token,
>;

impl ConstZero for GlobalSender {
    const ZEROED: Self = Self::new(
        TokenDeltaV3::ZEROED,
        [TokenDeltaV3::ZEROED; MAX_HARDCODED_DELTAS_V3],
        [TokenDeltaV3::ZEROED; MAX_CUSTOM_DELTAS_V3],
    );
}

impl GlobalSender {
    pub fn commit_side<T, In>(
        &mut self,
        token_index: T::TokenIndex,
        unsided_delta_atoms_per_lot: UnsidedDeltaAtomsPerLot,
        local_delta: &LocalDeltaV3,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        In: LegMatcher,
    {
        let delta = TokenDeltaV3::from_local_delta::<In>(unsided_delta_atoms_per_lot, local_delta);

        let delta_store = T::get_token_delta_v3(token_index, self);
        *delta_store = delta_store
            .checked_add(delta)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
