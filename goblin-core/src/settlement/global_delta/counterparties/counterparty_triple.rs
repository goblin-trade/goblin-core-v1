use crate::{
    axis::token::{CustomERC20, HardcodedERC20, Token, ETH},
    settlement::global_delta::CounterpartyMap,
    types::Triple,
};

pub type CounterpartyTriple = Triple<
    CounterpartyMap<ETH>,
    CounterpartyMap<HardcodedERC20>,
    CounterpartyMap<CustomERC20>,
    Token,
>;
