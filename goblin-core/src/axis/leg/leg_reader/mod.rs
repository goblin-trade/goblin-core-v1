use crate::{
    axis::{
        leg::{Base, Leg, LegMath, LegQuantities, Quote, SamePair},
        update::SameUpdatePair,
    },
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, FullPos, QuoteLots, QuoteLotsPerQuoteUnit, UnsidedAtoms,
        UnsidedAtomsPerLot, UnsidedLots,
    },
    settlement::LocalCounterparty,
    types::{StoreReader, Tuple},
};

pub trait LegReader: LegMath
    + StoreReader<
        Tuple<BaseLotsPerBaseUnit<u64>, QuoteLotsPerQuoteUnit<u64>, Leg>,
        Result = Self::LotsPerUnit,
    > + StoreReader<Tuple<BaseLots<u64>, QuoteLots<u64>, Leg>, Result = <Self as LegQuantities>::Lots>
    + StoreReader<
        Tuple<QuoteLots<u64>, BaseLots<u64>, Leg>,
        Result = <Self::Opposite as LegQuantities>::Lots,
    > + StoreReader<SamePair<FullPos>, Result = FullPos>
    + StoreReader<SamePair<UnsidedAtoms<i64>>, Result = UnsidedAtoms<i64>>
    + StoreReader<SamePair<UnsidedLots<i64>>, Result = UnsidedLots<i64>>
    + StoreReader<SamePair<UnsidedAtomsPerLot<u64>>, Result = UnsidedAtomsPerLot<u64>>
    + StoreReader<SamePair<UnsidedAtomsPerLot<i64>>, Result = UnsidedAtomsPerLot<i64>>
    + StoreReader<LocalCounterparty, Result = SameUpdatePair<UnsidedLots<u64>>>
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
