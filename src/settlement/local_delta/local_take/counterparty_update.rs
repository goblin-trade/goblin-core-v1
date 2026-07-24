use crate::{
    axis::{
        leg::{Pair, SamePair},
        update::UpdatePair,
    },
    quantities::UnsidedLots,
    settlement::ConstZero,
};

pub type CounterpartyUpdate = UpdatePair<UnsidedLots, UnsidedLots>;

impl ConstZero for CounterpartyUpdate {
    const ZEROED: Self = UpdatePair::new(UnsidedLots::ZEROED, UnsidedLots::ZEROED);
}

impl ConstZero for SamePair<CounterpartyUpdate> {
    const ZEROED: Self = Pair::new(CounterpartyUpdate::ZEROED, CounterpartyUpdate::ZEROED);
}
