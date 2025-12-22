use crate::{
    settlement::local_delta::DepositForSide,
    token::{TokenMarker, ERC20, ETH},
    types::{Base, LegMarker, Pair, Quote, TupleReader},
};

pub type LocalDepositStore = Pair<DepositForSide, DepositForSide>;

impl LocalDepositStore {
    pub const fn zero() -> Self {
        Self::new(DepositForSide::zero(), DepositForSide::zero())
    }

    fn deposit_for_side<T, In>(&mut self, deposit: T::Deposit)
    where
        T: TokenMarker
            + TupleReader<
                <ETH as TokenMarker>::Deposit,
                <ERC20 as TokenMarker>::Deposit,
                (ETH, ERC20),
                Result = <T as TokenMarker>::Deposit,
            >,
        In: LegMarker
            + TupleReader<DepositForSide, DepositForSide, (Base, Quote), Result = DepositForSide>,
    {
        let deposit_for_leg_marker = In::get_leg_mut(self);
        let deposit_for_token_marker = T::get_leg_mut(deposit_for_leg_marker);

        *deposit_for_token_marker = deposit;
    }

    pub fn set_deposits<B, Q>(&mut self, deposit_pair: &Pair<B::Deposit, Q::Deposit>)
    where
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
        // Map B to Base and Q to Quote
        // Rust limitation- we need to explicitly map the generic to its correct side
        self.deposit_for_side::<B, Base>(Base::get(deposit_pair));
        self.deposit_for_side::<Q, Quote>(Quote::get(deposit_pair));
    }

    pub fn reset<B, Q>(&mut self)
    where
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
        let deposit_pair = Pair::<B::Deposit, Q::Deposit>::default();
        self.set_deposits::<B, Q>(&deposit_pair);
    }
}
