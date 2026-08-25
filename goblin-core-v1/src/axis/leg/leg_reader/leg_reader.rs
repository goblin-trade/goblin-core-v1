use crate::{
    axis::leg::{leg_math::LegMath, leg_quantities::LegQuantities, Base, Leg, Quote, SamePair},
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, Position, QuoteLots, QuoteLotsPerQuoteUnit,
        UnsidedAtomsPerLot, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot, UnsidedDeltaLots,
    },
    settlement::local_delta::LocalCounterparty,
    types::{StoreReader, Tuple},
};

pub trait LegReader: LegMath
    // Can't add TokenIndexPair<B, Q> bound because it contains generics
    + StoreReader<Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
    + StoreReader<Tuple<BaseLots, QuoteLots, Leg>, Result = <Self as LegQuantities>::Lots>
    + StoreReader<Tuple<QuoteLots, BaseLots, Leg>, Result = <Self::Opposite as LegQuantities>::Lots>
    + StoreReader<SamePair<Position>, Result = Position>
    + StoreReader<SamePair<UnsidedDeltaAtoms>, Result = UnsidedDeltaAtoms>
    + StoreReader<SamePair<UnsidedDeltaLots>, Result = UnsidedDeltaLots>
    + StoreReader<SamePair<UnsidedAtomsPerLot>, Result = UnsidedAtomsPerLot>
    + StoreReader<SamePair<UnsidedDeltaAtomsPerLot>, Result = UnsidedDeltaAtomsPerLot>
    + StoreReader<SamePair<LocalCounterparty>, Result = LocalCounterparty>
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
