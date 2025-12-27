use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    market::{MarketHeader, MarketVariant},
    settlement::Delta,
    token::{TokenMarker, ERC20, ETH},
    types::{Pair, TupleReader},
};

impl<M, B, Q> MarketHeader<M, B, Q>
where
    M: MarketVariant,
    B: TokenMarker
        + TupleReader<
            <ETH as TokenMarker>::Deposit,
            <ERC20 as TokenMarker>::Deposit,
            (ETH, ERC20),
            Result = <B as TokenMarker>::Deposit,
        >,
    Q: TokenMarker
        + TupleReader<
            <ETH as TokenMarker>::Deposit,
            <ERC20 as TokenMarker>::Deposit,
            (ETH, ERC20),
            Result = <Q as TokenMarker>::Deposit,
        >,
{
    pub fn set_deposits(
        &self,
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        if self.decode_deposit_amounts {
            let base_deposit = B::Deposit::decode(args, offset, len)?;
            let quote_deposit = Q::Deposit::decode(args, offset, len)?;
            let deposit_pair = Pair::new(base_deposit, quote_deposit);

            delta.local.deposits.set_deposits::<B, Q>(&deposit_pair);
        }

        Ok(())
    }
}
