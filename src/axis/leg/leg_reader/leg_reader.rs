use crate::{
    axis::leg::{leg_math::LegMath, leg_quantities::LegQuantities, Base, Leg, Quote, SamePair},
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, DeltaAtoms, DeltaLots, Position, QuoteLots,
        QuoteLotsPerQuoteUnit, UnsidedDeltaAtomsPerLot,
    },
    settlement::local_delta::DepositTriple,
    types::{StoreReader, Tuple},
};

pub trait LegReader: LegMath
    + StoreReader<Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
    + StoreReader<Tuple<Position, Position, Leg>, Result = Position>
    + StoreReader<Tuple<DeltaAtoms, DeltaAtoms, Leg>, Result = DeltaAtoms>
    + StoreReader<Tuple<QuoteLots, BaseLots, Leg>, Result = <Self::Opposite as LegQuantities>::Lots>
    + StoreReader<Tuple<DepositTriple, DepositTriple, Leg>, Result = DepositTriple>
    + StoreReader<Tuple<DeltaLots, DeltaLots, Leg>, Result = DeltaLots>
    + StoreReader<SamePair<UnsidedDeltaAtomsPerLot>, Result = UnsidedDeltaAtomsPerLot>
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
