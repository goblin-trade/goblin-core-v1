use crate::{
    axis::leg::{leg_quantities::LegQuantities, Base, Leg, Quote},
    quantities::{BaseLotsPerBaseUnit, DeltaAtoms, Position, QuoteLotsPerQuoteUnit},
    types::{StoreReader, Tuple},
};

pub trait LegReader: LegQuantities
    + StoreReader<Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
    + StoreReader<Tuple<Position, Position, Leg>, Result = Position>
    + StoreReader<Tuple<DeltaAtoms, DeltaAtoms, Leg>, Result = DeltaAtoms>
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
