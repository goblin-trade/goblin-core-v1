use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_to_token::LegToToken, Base, Pair, Quote, SamePair},
        token::{token_marker::TokenMarker, token_quantity::TokenQuantity},
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::{local_delta::DepositTriple, ConstZero},
    types::StoreReader,
};

/// Local deposits for a market
pub type LocalDeposits = SamePair<DepositTriple>;

impl ConstZero for LocalDeposits {
    const ZEROED: Self = Pair::new(DepositTriple::ZEROED, DepositTriple::ZEROED);
}

impl LocalDeposits {
    // TODO this should set pair
    // Read externally from ctx
    pub fn set_leg<B, Q, In>(&mut self, ctx: &DecodeCtx) -> Result<(), GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher + LegToToken<B, Q>,
    {
        let deposit_store = In::Selected::get_leg_mut(In::get_leg_mut(self));
        *deposit_store = <In::Selected as TokenQuantity>::LocalDeposit::try_decode(ctx)?;

        Ok(())
    }

    fn set_leg_deposit_deprecated<T, In>(&mut self, deposit: T::LocalDeposit)
    where
        In: LegMatcher,
        T: TokenMarker,
    {
        let leg_deposits = In::get_leg_mut(self);
        let token_variant_deposit = T::get_leg_mut(leg_deposits);

        *token_variant_deposit = deposit;
    }

    pub fn read_deposits<'a, B, Q>(&mut self, ctx: &DecodeCtx) -> Result<(), GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        let base_deposit = B::LocalDeposit::try_decode(ctx)?;
        let quote_deposit = Q::LocalDeposit::try_decode(ctx)?;

        self.set_leg_deposit_deprecated::<B, Base>(base_deposit);
        self.set_leg_deposit_deprecated::<Q, Quote>(quote_deposit);

        Ok(())
    }
    pub fn reset<B, Q>(&mut self)
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        self.set_leg_deposit_deprecated::<B, Base>(B::LocalDeposit::ZEROED);
        self.set_leg_deposit_deprecated::<Q, Quote>(Q::LocalDeposit::ZEROED);
    }
}
