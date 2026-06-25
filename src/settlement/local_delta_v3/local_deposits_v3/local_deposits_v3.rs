use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Pair, Quote, SamePair},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::{local_delta_v3::DepositTripleV3, ConstZero},
};

/// Local deposits for a market
pub type LocalDepositsV3 = SamePair<DepositTripleV3>;

impl ConstZero for LocalDepositsV3 {
    const ZEROED: Self = Pair::new(DepositTripleV3::ZEROED, DepositTripleV3::ZEROED);
}

impl LocalDepositsV3 {
    pub fn read_deposits<'a, B, Q>(&mut self, ctx: &DecodeCtx) -> Result<(), GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        let base_deposit = B::LocalDeposit::try_decode(ctx)?;
        let quote_deposit = Q::LocalDeposit::try_decode(ctx)?;

        self.set_side_deposit::<B, Base>(base_deposit);
        self.set_side_deposit::<Q, Quote>(quote_deposit);

        Ok(())
    }

    pub fn reset<B, Q>(&mut self)
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        self.set_side_deposit::<B, Base>(B::LocalDeposit::default());
        self.set_side_deposit::<Q, Quote>(Q::LocalDeposit::default());
    }

    fn set_side_deposit<T, In>(&mut self, deposit: T::LocalDeposit)
    where
        In: LegMatcher,
        T: TokenMarker,
    {
        let leg_deposits = In::get_leg_mut(self);
        let token_variant_deposit = T::get_leg_mut(leg_deposits);

        *token_variant_deposit = deposit;
    }
}
