use crate::{
    axis::token::{CustomERC20, HardcodedERC20, Token, ETH},
    settlement::{global_delta_v3::CounterpartyMap, ConstZero},
    types::Triple,
};

pub type Counterparties = Triple<
    CounterpartyMap<ETH>,
    CounterpartyMap<HardcodedERC20>,
    CounterpartyMap<CustomERC20>,
    Token,
>;

impl ConstZero for Counterparties {
    const ZEROED: Self = Self::new(
        CounterpartyMap::ZEROED,
        CounterpartyMap::ZEROED,
        CounterpartyMap::ZEROED,
    );
}
