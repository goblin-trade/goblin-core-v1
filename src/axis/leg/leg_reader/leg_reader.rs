use crate::{
    axis::leg::{leg_quantities::LegQuantities, Base, Leg, Quote},
    quantities::{BaseLotsPerBaseUnit, DeltaAtoms, QuoteLotsPerQuoteUnit, SafePosition, POS_2},
    types::{StoreReader, Tuple},
};

pub trait LegReader: LegQuantities
    + StoreReader<Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
    + StoreReader<Tuple<SafePosition<POS_2>, SafePosition<POS_2>, Leg>, Result = SafePosition<POS_2>>
    + StoreReader<Tuple<DeltaAtoms, DeltaAtoms, Leg>, Result = DeltaAtoms>
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
