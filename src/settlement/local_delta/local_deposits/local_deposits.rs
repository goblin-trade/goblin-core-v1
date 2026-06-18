use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Pair, Quote, SamePair},
        token::token_reader::TokenReader,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::{local_delta::DepositTriple, ConstZero},
};

/// Local deposits for a market
pub type LocalDeposits = SamePair<DepositTriple>;

impl ConstZero for LocalDeposits {
    const ZEROED: Self = Pair::new(DepositTriple::ZEROED, DepositTriple::ZEROED);
}

impl LocalDeposits {
    pub fn read_deposits<'a, B, Q>(&mut self, ctx: &DecodeCtx) -> Result<(), GoblinError>
    where
        B: TokenReader,
        Q: TokenReader,
    {
        let base_deposit = B::Deposit::try_decode(ctx)?;
        let quote_deposit = Q::Deposit::try_decode(ctx)?;

        self.set_side_deposit::<B, Base>(base_deposit);
        self.set_side_deposit::<Q, Quote>(quote_deposit);

        Ok(())
    }

    pub fn reset<B, Q>(&mut self)
    where
        B: TokenReader,
        Q: TokenReader,
    {
        self.set_side_deposit::<B, Base>(B::Deposit::default());
        self.set_side_deposit::<Q, Quote>(Q::Deposit::default());
    }

    fn set_side_deposit<T, In>(&mut self, deposit: T::Deposit)
    where
        T: TokenReader,
        In: LegMatcher,
    {
        let leg_deposits = In::get_leg_mut(self);
        let token_variant_deposit = T::get_leg_mut(leg_deposits);

        *token_variant_deposit = deposit;
    }
}
