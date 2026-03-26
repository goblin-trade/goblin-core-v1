use crate::{
    axis::leg::{leg_quantities::LegQuantities, Base, Leg, Quote},
    matching::bitmap::StoredCoordinates,
    quantities::{BaseLots, BaseLotsPerBaseUnit, DeltaAtoms, QuoteLots, QuoteLotsPerQuoteUnit},
    types::{StoreReader, Tuple},
};

pub trait LegReader: LegQuantities
    + StoreReader<Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
    + StoreReader<Tuple<StoredCoordinates, StoredCoordinates, Leg>, Result = StoredCoordinates>
    + StoreReader<Tuple<DeltaAtoms, DeltaAtoms, Leg>, Result = DeltaAtoms>
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
