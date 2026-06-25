use crate::{
    axis::leg::{leg_math::LegMath, leg_quantities::LegQuantities, Base, Leg, Quote},
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, DeltaAtoms, Position, QuoteLots, QuoteLotsPerQuoteUnit,
    },
    settlement::{
        local_delta::DepositTriple, local_delta_v3::DepositTripleV3, SidedSenderDeltaV2,
        SidedTakeDeltaV2,
    },
    types::{StoreReader, Tuple},
};

pub trait LegReader: LegMath
    + StoreReader<Tuple<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Leg>, Result = Self::LotsPerUnit>
    + StoreReader<Tuple<Position, Position, Leg>, Result = Position>
    + StoreReader<Tuple<DeltaAtoms, DeltaAtoms, Leg>, Result = DeltaAtoms>
    + StoreReader<Tuple<QuoteLots, BaseLots, Leg>, Result = <Self::Opposite as LegQuantities>::Lots>
    + StoreReader<
        Tuple<SidedSenderDeltaV2<Base>, SidedSenderDeltaV2<Quote>, Leg>,
        Result = SidedSenderDeltaV2<Self>,
    > + StoreReader<
        Tuple<SidedTakeDeltaV2<Base>, SidedTakeDeltaV2<Quote>, Leg>,
        Result = SidedTakeDeltaV2<Self>,
    > + StoreReader<Tuple<DepositTriple, DepositTriple, Leg>, Result = DepositTriple>
    + StoreReader<
        Tuple<DepositTripleV3<Base>, DepositTripleV3<Quote>, Leg>,
        Result = DepositTripleV3<Self>,
    >
{
}

impl LegReader for Base {}
impl LegReader for Quote {}
