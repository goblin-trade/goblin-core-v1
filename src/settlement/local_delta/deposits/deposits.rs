use crate::{
    axis::{
        leg::{Base, Pair, Quote, SamePair},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    quantities::DeltaAtoms,
    settlement::ConstZero,
    types::StoreReader,
};

/// ERC20 deposits for base and quote token for a given market.
///
/// ETH cannot be dynamically deposited. We only track deposit amounts for
/// ERC20 tokens. The code is common for both HardcodedERC20 and CustomERC20
pub type Deposits = SamePair<DeltaAtoms>;

impl ConstZero for Deposits {
    const ZEROED: Self = Pair::new(DeltaAtoms::ZEROED, DeltaAtoms::ZEROED);
}

impl Deposits {
    pub fn set_deposits<'a, B, Q>(&mut self, ctx: &DecodeCtx) -> Result<(), GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        let base_deposit = B::Deposit::try_decode(ctx)?;
        let quote_deposit = Q::Deposit::try_decode(ctx)?;

        *Base::get_leg_mut(self) = base_deposit.into();
        *Quote::get_leg_mut(self) = quote_deposit.into();

        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::ZEROED;
    }
}
