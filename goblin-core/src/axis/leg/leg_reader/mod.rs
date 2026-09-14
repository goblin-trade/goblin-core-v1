use crate::{
    axis::{
        leg::{Base, Leg, LegMath, LegQuantities, Quote, SamePair},
        update::SameUpdatePair,
    },
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, Position, QuoteLots, QuoteLotsPerQuoteUnit, UnsidedAtoms,
        UnsidedAtomsPerLot, UnsidedDeltaAtomsPerLot, UnsidedLots,
    },
    settlement::LocalCounterparty,
    types::{StoreReader, Tuple},
};

pub trait LegReader: LegMath
    // Can't add TokenIndexPair<B, Q> bound because it contains generics
    + StoreReader<Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
    + StoreReader<Tuple<BaseLots, QuoteLots, Leg>, Result = <Self as LegQuantities>::Lots>
    + StoreReader<Tuple<QuoteLots, BaseLots, Leg>, Result = <Self::Opposite as LegQuantities>::Lots>
    + StoreReader<SamePair<Position>, Result = Position>
    + StoreReader<SamePair<UnsidedAtoms<i64>>, Result = UnsidedAtoms<i64>>
    + StoreReader<SamePair<UnsidedLots<i64>>, Result = UnsidedLots<i64>>
    + StoreReader<SamePair<UnsidedAtomsPerLot>, Result = UnsidedAtomsPerLot>
    + StoreReader<SamePair<UnsidedDeltaAtomsPerLot>, Result = UnsidedDeltaAtomsPerLot>
    + StoreReader<LocalCounterparty, Result = SameUpdatePair<UnsidedLots<u64>>>
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
