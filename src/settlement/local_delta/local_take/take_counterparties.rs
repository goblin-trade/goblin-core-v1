use crate::{
    axis::leg::SamePair,
    settlement::{local_delta::local_take::CounterpartyUpdate, ConstZero},
    types::{Address, FixedMap},
};

pub const MAX_COUNTERPARTY: usize = 16;

pub type TakeCounterparties = FixedMap<Address, SamePair<CounterpartyUpdate>, MAX_COUNTERPARTY>;

impl ConstZero for TakeCounterparties {
    const ZEROED: Self = Self {
        entries: [(Address::ZEROED, SamePair::<CounterpartyUpdate>::ZEROED); MAX_COUNTERPARTY],
        len: 0,
    };
}
