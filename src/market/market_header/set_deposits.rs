use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::{MarketHeader, MarketVariant},
    quantities::DeltaAtoms,
    settlement::Delta,
    token::{HardcodedERC20, TokenMarker, ERC20, ETH},
    types::{Pair, TupleMarker, TupleReader},
};

impl<'a, M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketVariant,
    B: TokenMarker,
    B::TupleMarker: TupleReader<
        (),
        DeltaAtoms,
        (ETH, ERC20),
        Result = <B::TupleMarker as TupleMarker>::Deposit,
    >,
    Q: TokenMarker,
    Q::TupleMarker: TupleReader<
        (),
        DeltaAtoms,
        (ETH, ERC20),
        Result = <Q::TupleMarker as TupleMarker>::Deposit,
    >,
    B::Deposit: Decodable<'a>,
    Q::Deposit: Decodable<'a>,
{
    pub fn set_deposits(
        &self,
        ctx: &'a DecodeCtx<'a>,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        if self.decode_deposit_amounts {
            let base_deposit = B::Deposit::try_decode(ctx)?;
            let quote_deposit = Q::Deposit::try_decode(ctx)?;
            let deposit_pair = Pair::new(base_deposit, quote_deposit);

            // delta.local.deposits.set_deposits::<B, Q>(&deposit_pair);
        }

        Ok(())
    }
}
