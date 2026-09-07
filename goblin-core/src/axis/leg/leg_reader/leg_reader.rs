use crate::{
    axis::{
        leg::{Base, Leg, LegMath, LegQuantities, Quote, SamePair},
        update::SameUpdatePair,
    },
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, Position, QuoteLots, QuoteLotsPerQuoteUnit,
        UnsidedAtomsPerLot, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot, UnsidedDeltaLots,
        UnsidedLots,
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
    + StoreReader<SamePair<UnsidedDeltaAtoms>, Result = UnsidedDeltaAtoms>
    + StoreReader<SamePair<UnsidedDeltaLots>, Result = UnsidedDeltaLots>
    + StoreReader<SamePair<UnsidedAtomsPerLot>, Result = UnsidedAtomsPerLot>
    + StoreReader<SamePair<UnsidedDeltaAtomsPerLot>, Result = UnsidedDeltaAtomsPerLot>
    + StoreReader<LocalCounterparty, Result = SameUpdatePair<UnsidedLots>>
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
