use crate::{
    axis::leg::{leg_quantities::LegQuantities, Base, Leg, Quote},
    quantities::{BaseLotsPerBaseUnit, DeltaAtoms, Pos2, QuoteLotsPerQuoteUnit},
    types::{StoreReader, Tuple},
};

pub trait LegReader: LegQuantities
    + StoreReader<Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
    + StoreReader<Tuple<Pos2, Pos2, Leg>, Result = Pos2>
    + StoreReader<Tuple<DeltaAtoms, DeltaAtoms, Leg>, Result = DeltaAtoms>
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
