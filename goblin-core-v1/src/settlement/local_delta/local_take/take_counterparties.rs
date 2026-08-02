use crate::{
    axis::leg::SamePair,
    settlement::{local_delta::local_take::LocalCounterparty, ConstZero},
    types::{Address, FixedMap},
};

pub const MAX_COUNTERPARTY: usize = 16;

pub type TakeCounterparties = FixedMap<Address, SamePair<LocalCounterparty>, MAX_COUNTERPARTY>;

impl ConstZero for TakeCounterparties {
    const ZEROED: Self = Self {
        entries: [(Address::ZEROED, SamePair::<LocalCounterparty>::ZEROED); MAX_COUNTERPARTY],
        len: 0,
    };
}
