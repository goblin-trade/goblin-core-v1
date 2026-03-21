use crate::{axis::leg::leg_matcher::LegMatcher, matching::bitmap::inner_pos::InnerPos};

pub struct UpdateHeader<In>
where
    In: LegMatcher,
{
    pub inner_pos: InnerPos<In>,
}
