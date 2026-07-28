use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_to_token::LegToToken, SamePair},
        token::{token_marker::TokenMarker, token_quantity::TokenQuantity},
    },
    for_axes,
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::{
        local_delta::{DepositPair, DepositTriple},
        ConstZero,
    },
    types::StoreReader,
};

/// Local deposits for a market
pub type LocalDeposits = SamePair<DepositTriple>;

impl LocalDeposits {
    pub fn decode_and_set<'a, B, Q>(&mut self, ctx: &DecodeCtx) -> Result<(), GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        let deposit_pair = DepositPair::<B, Q>::try_decode(ctx)?;
        for_axes!(|In| self.set_leg::<B, Q, In>(&deposit_pair));

        Ok(())
    }
    pub fn reset<B, Q>(&mut self)
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        for_axes!(|In| self.set_leg::<B, Q, In>(&DepositPair::<B, Q>::ZEROED));
    }

    fn set_leg<B, Q, In>(&mut self, deposit_pair: &DepositPair<B, Q>)
    where
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher
            + LegToToken<B, Q>
            + StoreReader<DepositPair<B, Q>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
    {
        // TODO nested StoreReader utility
        let deposit_store = In::Selected::get_leg_mut(In::get_leg_mut(self));
        let deposit = In::get(deposit_pair);
        *deposit_store = deposit;
    }
}
