use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_to_token::LegToToken, SamePair},
        token::token_quantity::TokenQuantity,
    },
    for_axes,
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    market::TokenPair,
    settlement::local_delta::{DepositPair, DepositTriple},
    types::StoreReader,
};

/// Local deposits for a market
pub type LocalDeposits = SamePair<DepositTriple>;

impl LocalDeposits {
    pub fn decode_and_set<'a, TP: TokenPair>(
        &mut self,
        ctx: &DecodeCtx,
    ) -> Result<(), GoblinError> {
        let deposit_pair = DepositPair::<TP>::try_fixed_decode(ctx)?;
        for_axes!(In => self.set_leg::<TP, In>(&deposit_pair));
        Ok(())
    }

    fn set_leg<TP, In>(&mut self, deposit_pair: &DepositPair<TP>)
    where
        TP: TokenPair,
        In: LegMatcher
            + LegToToken<TP>
            + StoreReader<DepositPair<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
    {
        let deposit_triple = In::get_leg_mut(self);
        let deposit_store = In::Selected::get_leg_mut(deposit_triple);

        *deposit_store = In::get(deposit_pair);
    }
}
