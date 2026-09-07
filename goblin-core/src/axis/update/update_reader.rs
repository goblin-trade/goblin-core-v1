use crate::{
    axis::{
        leg::leg_math::LegMath,
        update::{Decrease, Increase, Update},
    },
    types::{StoreReader, Tuple},
};

pub trait UpdateReader<In>:
    StoreReader<
    Tuple<<In::Opposite as LegMath>::MatchingLots, <In::Opposite as LegMath>::MatchingLots, Update>,
    Result = <In::Opposite as LegMath>::MatchingLots,
>
where
    In: LegMath,
{
}

impl<In> UpdateReader<In> for Increase where In: LegMath {}
impl<In> UpdateReader<In> for Decrease where In: LegMath {}
