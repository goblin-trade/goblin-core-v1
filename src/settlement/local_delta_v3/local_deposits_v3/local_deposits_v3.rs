use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base, Pair, Quote, SamePair},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::{local_delta_v3::DepositTripleV3, ConstZero},
    types::StoreReader,
};

/// Local deposits for a market
pub type LocalDepositsV3 = Pair<DepositTripleV3<Base>, DepositTripleV3<Quote>>;

impl ConstZero for LocalDepositsV3 {
    const ZEROED: Self = Pair::new(
        DepositTripleV3::<Base>::ZEROED,
        DepositTripleV3::<Quote>::ZEROED,
    );
}

impl LocalDepositsV3 {
    pub fn read_deposits<'a, B, Q>(&mut self, ctx: &DecodeCtx) -> Result<(), GoblinError>
    where
        B: TokenMarker + StoreReader<DepositTripleV3<Base>, Result = B::LocalDeposit<Base>>,
        Q: TokenMarker + StoreReader<DepositTripleV3<Quote>, Result = Q::LocalDeposit<Quote>>,
    {
        let base_deposit = B::LocalDeposit::try_decode(ctx)?;
        let quote_deposit = Q::LocalDeposit::try_decode(ctx)?;

        self.set_side_deposit::<B, Base>(base_deposit);
        self.set_side_deposit::<Q, Quote>(quote_deposit);

        Ok(())
    }

    pub fn reset<B, Q>(&mut self)
    where
        B: TokenMarker + StoreReader<DepositTripleV3<Base>, Result = B::LocalDeposit<Base>>,
        Q: TokenMarker + StoreReader<DepositTripleV3<Quote>, Result = Q::LocalDeposit<Quote>>,
    {
        self.set_side_deposit::<B, Base>(B::LocalDeposit::<Base>::default());
        self.set_side_deposit::<Q, Quote>(Q::LocalDeposit::<Quote>::default());
    }

    fn set_side_deposit<T, In>(&mut self, deposit: T::LocalDeposit<In>)
    where
        In: LegMatcher,
        T: TokenMarker + StoreReader<DepositTripleV3<In>, Result = T::LocalDeposit<In>>,
    {
        let leg_deposits = In::get_leg_mut(self);
        let token_variant_deposit = T::get_leg_mut(leg_deposits);

        *token_variant_deposit = deposit;
    }
}
