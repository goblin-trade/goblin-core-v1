use crate::{axis::CallerEnum, axis_helpers::AxisMarker};

pub trait CallerMarker: AxisMarker<Enum = CallerEnum> {
    type CallerIndex;
}
