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
pub type LocalDepositsV3 = Pair<DepositTripleV3<Base>, DepositTripleV3<Quote>>;

impl ConstZero for LocalDepositsV3 {
    const ZEROED: Self = Pair::new(DepositTripleV3::ZEROED, DepositTripleV3::ZEROED);
}

// impl LocalDeposits {
//     pub fn read_deposits<'a, B, Q>(&mut self, ctx: &DecodeCtx) -> Result<(), GoblinError>
//     where
//         B: TokenMarker,
//         Q: TokenMarker,
//     {
//         let base_deposit = B::GlobalDeposit::try_decode(ctx)?;
//         let quote_deposit = Q::GlobalDeposit::try_decode(ctx)?;

//         self.set_side_deposit::<B, Base>(base_deposit);
//         self.set_side_deposit::<Q, Quote>(quote_deposit);

//         Ok(())
//     }

//     pub fn reset<B, Q>(&mut self)
//     where
//         B: TokenMarker,
//         Q: TokenMarker,
//     {
//         self.set_side_deposit::<B, Base>(B::GlobalDeposit::default());
//         self.set_side_deposit::<Q, Quote>(Q::GlobalDeposit::default());
//     }

//     fn set_side_deposit<T, In>(&mut self, deposit: T::GlobalDeposit)
//     where
//         T: TokenMarker,
//         In: LegMatcher,
//     {
//         let leg_deposits = In::get_leg_mut(self);
//         let token_variant_deposit = T::get_leg_mut(leg_deposits);

//         *token_variant_deposit = deposit;
//     }
// }
