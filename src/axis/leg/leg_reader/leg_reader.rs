use crate::{
    axis::leg::{leg_math::LegMath, leg_quantities::LegQuantities, Base, Leg, Quote, SamePair},
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, Position, QuoteLots, QuoteLotsPerQuoteUnit,
        UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot, UnsidedDeltaLots,
    },
    settlement::local_delta::{local_take::CounterpartyUpdate, DepositTriple},
    types::{StoreReader, Tuple},
};

pub trait LegReader: LegMath
    + StoreReader<Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
    + StoreReader<Tuple<Position, Position, Leg>, Result = Position>
    + StoreReader<Tuple<UnsidedDeltaAtoms, UnsidedDeltaAtoms, Leg>, Result = UnsidedDeltaAtoms>
    + StoreReader<Tuple<QuoteLots, BaseLots, Leg>, Result = <Self::Opposite as LegQuantities>::Lots>
    + StoreReader<Tuple<DepositTriple, DepositTriple, Leg>, Result = DepositTriple>
    + StoreReader<Tuple<UnsidedDeltaLots, UnsidedDeltaLots, Leg>, Result = UnsidedDeltaLots>
    + StoreReader<SamePair<UnsidedDeltaAtomsPerLot>, Result = UnsidedDeltaAtomsPerLot> // TODO remove
    + StoreReader<SamePair<CounterpartyUpdate>, Result = CounterpartyUpdate>
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
