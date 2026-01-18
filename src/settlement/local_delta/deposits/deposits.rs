use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    quantities::DeltaAtoms,
    token::TokenMarker,
    types::{Base, Pair, Quote},
};

/// ERC20 deposits for base and quote token for a given market.
///
/// ETH cannot be dynamically deposited. We only track deposit amounts for
/// ERC20 tokens. The code is common for both HardcodedERC20 and CustomERC20
pub type Deposits = Pair<DeltaAtoms, DeltaAtoms>;

impl Deposits {
    pub fn set_deposits<'a, B, Q>(&mut self, ctx: &'a DecodeCtx<'a>) -> Result<(), GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        B::Deposit: Decodable<'a>,
        Q::Deposit: Decodable<'a>,
    {
        let base_deposit = B::Deposit::try_decode(ctx)?;
        let quote_deposit = Q::Deposit::try_decode(ctx)?;

        B::set_deposit::<Base>(self, base_deposit);
        Q::set_deposit::<Quote>(self, quote_deposit);

        Ok(())
    }

    pub fn reset<B, Q>(&mut self)
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        B::set_deposit::<Base>(self, B::Deposit::default());
        Q::set_deposit::<Quote>(self, Q::Deposit::default());
    }
}
