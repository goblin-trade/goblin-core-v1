use crate::{
    axis::leg::{
        leg_constants::LegConstants, leg_coordinates::LegCoordinates, leg_iterator::LegIterator,
        leg_math::LegMath, leg_quantities::LegQuantities, leg_reader::LegReader,
        leg_validator::LegValidator, Base, Leg, Quote,
    },
    quantities::{BaseLots, QuoteLots},
    settlement::sender_delta::{SidedSenderDeltaV2, SidedTakeDeltaV2},
    types::{StoreReader, Tuple},
};

/// Supertrait for leg operations
pub trait LegMatcher:
    LegQuantities
    + LegMath
    + LegConstants
    + LegValidator
    + LegCoordinates
    + LegIterator
    + LegReader
    + StoreReader<Tuple<QuoteLots, BaseLots, Leg>, Result = <Self::Opposite as LegQuantities>::Lots>
    + StoreReader<
        Tuple<SidedSenderDeltaV2<Base>, SidedSenderDeltaV2<Quote>, Leg>,
        Result = SidedSenderDeltaV2<Self>,
    > + StoreReader<
        Tuple<SidedTakeDeltaV2<Base>, SidedTakeDeltaV2<Quote>, Leg>,
        Result = SidedTakeDeltaV2<Self>,
    >
{
}

impl LegMatcher for Base {}
impl LegMatcher for Quote {}
