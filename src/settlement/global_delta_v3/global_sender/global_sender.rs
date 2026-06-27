use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::LotSizePair,
        token::{
            token_delta_manager::TokenDeltaManager,
            token_marker::hardcoded_erc20::HARDCODED_TOKENS, CustomERC20, HardcodedERC20, Token,
            ETH,
        },
        update::Increase,
    },
    goblin_error::GoblinError,
    quantities::{TryIntoUnsidedDelta, UnsideQuantity, UnsidedDeltaAtomsPerLot},
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
    pub fn commit_side<In, T>(
        &mut self,
        token_index: T::TokenIndex,
        lot_size_pair: &LotSizePair,
        local_delta: &LocalDeltaV3,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
        T: TokenDeltaManager,
    {
        let lot_size = In::get(lot_size_pair);
        let atoms_per_lot = In::atoms_per_lot(lot_size);
        let unsided_delta_atoms_per_lot = atoms_per_lot.try_into_unsided_delta::<Increase>()?;

        let delta = TokenDeltaV3::from_local_delta::<In>(unsided_delta_atoms_per_lot, local_delta);

        let delta_store = T::get_token_delta_v3(token_index, self);
        *delta_store = delta_store
            .checked_add(delta)
            .ok_or(GoblinError::DeltaOverflow)?;

        Ok(())
    }
}
