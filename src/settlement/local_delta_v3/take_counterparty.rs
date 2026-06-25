use crate::{
    settlement::{local_delta_v3::DeltaLotsPair, ConstZero},
    types::{Address, FixedMap},
};

pub const MAX_COUNTERPARTY: usize = 16;

pub type TakeCounterparty = FixedMap<Address, DeltaLotsPair, MAX_COUNTERPARTY>;

impl ConstZero for TakeCounterparty {
    const ZEROED: Self = Self {
        entries: [(Address::ZEROED, DeltaLotsPair::ZEROED); MAX_COUNTERPARTY],
        len: 0,
    };
}
